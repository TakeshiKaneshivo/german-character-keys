import CoreGraphics
import Foundation
import ImageIO

guard CommandLine.arguments.count == 6 else {
    fputs("usage: generate-icons.swift <disabled-source> <enabled-source> <icons-directory> <public-images-directory> <nsis-directory>\n", stderr)
    exit(2)
}

let disabledSourceURL = URL(fileURLWithPath: CommandLine.arguments[1])
let enabledSourceURL = URL(fileURLWithPath: CommandLine.arguments[2])
let iconsDirectory = URL(fileURLWithPath: CommandLine.arguments[3])
let publicImagesDirectory = URL(fileURLWithPath: CommandLine.arguments[4])
let nsisDirectory = URL(fileURLWithPath: CommandLine.arguments[5])
let mainIconSize = 512
let whiteBackgroundNoiseAlphaCutoff = 8
let trayInset1x = 1
let trayInset2x = 2

func loadImage(from url: URL) -> CGImage {
    guard let source = CGImageSourceCreateWithURL(url as CFURL, nil),
          let image = CGImageSourceCreateImageAtIndex(source, 0, nil) else {
        fatalError("unable to read source image: \(url.path)")
    }
    return image
}

func makeContext(width: Int, height: Int) -> CGContext {
    CGContext(
        data: nil,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: width * 4,
        space: CGColorSpaceCreateDeviceRGB(),
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
    )!
}

func image(from context: CGContext) -> CGImage {
    context.makeImage()!
}

func writePNG(_ image: CGImage, to url: URL) {
    guard let destination = CGImageDestinationCreateWithURL(url as CFURL, "public.png" as CFString, 1, nil) else {
        fatalError("unable to create PNG destination: \(url.path)")
    }
    CGImageDestinationAddImage(destination, image, nil)
    guard CGImageDestinationFinalize(destination) else {
        fatalError("unable to write PNG: \(url.path)")
    }
}

func uint16LE(_ value: UInt16) -> [UInt8] {
    [UInt8(value & 0xff), UInt8((value >> 8) & 0xff)]
}

func uint32LE(_ value: UInt32) -> [UInt8] {
    [
        UInt8(value & 0xff),
        UInt8((value >> 8) & 0xff),
        UInt8((value >> 16) & 0xff),
        UInt8((value >> 24) & 0xff),
    ]
}

func int32LE(_ value: Int32) -> [UInt8] {
    uint32LE(UInt32(bitPattern: value))
}

func normalizeToBlackAlpha(_ source: CGImage, canvasSize: Int) -> CGContext {
    let context = makeContext(width: canvasSize, height: canvasSize)
    context.clear(CGRect(x: 0, y: 0, width: canvasSize, height: canvasSize))
    context.interpolationQuality = .high

    let sourceWidth = CGFloat(source.width)
    let sourceHeight = CGFloat(source.height)
    let scale = min(CGFloat(canvasSize) / sourceWidth, CGFloat(canvasSize) / sourceHeight)
    let drawSize = CGSize(width: sourceWidth * scale, height: sourceHeight * scale)
    context.draw(source, in: CGRect(
        x: (CGFloat(canvasSize) - drawSize.width) / 2,
        y: (CGFloat(canvasSize) - drawSize.height) / 2,
        width: drawSize.width,
        height: drawSize.height
    ))

    // Convert both transparent black artwork and opaque white-background
    // artwork to the same black-on-alpha representation.
    let pixels = context.data!.assumingMemoryBound(to: UInt8.self)
    for index in stride(from: 0, to: canvasSize * canvasSize * 4, by: 4) {
        let sourceAlpha = Int(pixels[index + 3])
        let luminance = (Int(pixels[index]) + Int(pixels[index + 1]) + Int(pixels[index + 2])) / 3
        let extractedAlpha = max(0, 255 - luminance)
        var alpha = extractedAlpha * sourceAlpha / 255
        if sourceAlpha > 250 && extractedAlpha <= whiteBackgroundNoiseAlphaCutoff {
            alpha = 0
        }
        pixels[index] = 0
        pixels[index + 1] = 0
        pixels[index + 2] = 0
        pixels[index + 3] = UInt8(alpha)
    }
    return context
}

func alphaBounds(_ context: CGContext, threshold: UInt8 = 8) -> CGRect {
    let pixels = context.data!.assumingMemoryBound(to: UInt8.self)
    var minX = context.width
    var minY = context.height
    var maxX = -1
    var maxY = -1

    for y in 0..<context.height {
        for x in 0..<context.width {
            let alpha = pixels[(y * context.bytesPerRow) + x * 4 + 3]
            guard alpha > threshold else { continue }
            minX = min(minX, x)
            minY = min(minY, y)
            maxX = max(maxX, x)
            maxY = max(maxY, y)
        }
    }

    guard maxX >= minX, maxY >= minY else {
        fatalError("source image has no visible alpha pixels")
    }
    return CGRect(x: minX, y: minY, width: maxX - minX + 1, height: maxY - minY + 1)
}

func makeTrayIcon(source: CGImage, cropBounds: CGRect, size: Int, inset: Int) -> CGImage {
    guard let cropped = source.cropping(to: cropBounds) else {
        fatalError("unable to crop tray source")
    }

    let context = makeContext(width: size, height: size)
    context.clear(CGRect(x: 0, y: 0, width: size, height: size))
    context.interpolationQuality = .high

    let target = CGFloat(size - inset * 2)
    let scale = min(target / CGFloat(cropped.width), target / CGFloat(cropped.height))
    let drawSize = CGSize(
        width: CGFloat(cropped.width) * scale,
        height: CGFloat(cropped.height) * scale
    )
    context.draw(cropped, in: CGRect(
        x: (CGFloat(size) - drawSize.width) / 2,
        y: (CGFloat(size) - drawSize.height) / 2,
        width: drawSize.width,
        height: drawSize.height
    ))

    // Template resources must contain black RGB values only; macOS uses alpha
    // to tint them for the current menu-bar appearance.
    let pixels = context.data!.assumingMemoryBound(to: UInt8.self)
    for index in stride(from: 0, to: size * size * 4, by: 4) {
        pixels[index] = 0
        pixels[index + 1] = 0
        pixels[index + 2] = 0
    }
    return image(from: context)
}

func validateTemplate(_ image: CGImage, name: String) {
    guard let raw = image.dataProvider?.data else {
        fatalError("unable to inspect \(name)")
    }
    let bytes = [UInt8](raw as Data)
    let bytesPerPixel = image.bitsPerPixel / 8
    var visible = 0
    var nonBlack = 0

    for y in 0..<image.height {
        for x in 0..<image.width {
            let offset = y * image.bytesPerRow + x * bytesPerPixel
            let alpha = bytesPerPixel >= 4 ? bytes[offset + 3] : 255
            guard alpha > 8 else { continue }
            visible += 1
            if bytes[offset] != 0 || bytes[offset + 1] != 0 || bytes[offset + 2] != 0 {
                nonBlack += 1
            }
        }
    }

    guard visible > 0, nonBlack == 0 else {
        fatalError("invalid template image \(name): visible=\(visible), nonBlack=\(nonBlack)")
    }
}

func fill(_ context: CGContext, x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat, red: CGFloat, green: CGFloat, blue: CGFloat) {
    context.setFillColor(CGColor(red: red / 255, green: green / 255, blue: blue / 255, alpha: 1))
    context.fill(CGRect(x: x, y: y, width: width, height: height))
}

func makeInstallerBitmap(width: Int, height: Int, draw: (CGContext) -> Void) -> CGImage {
    let context = makeContext(width: width, height: height)
    context.interpolationQuality = .high
    draw(context)
    return image(from: context)
}

func writeBMP(_ image: CGImage, to url: URL) {
    let width = image.width
    let height = image.height
    let context = makeContext(width: width, height: height)
    context.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))

    let pixelBytes = width * height * 4
    let fileHeaderSize = 14
    let dibHeaderSize = 40
    let pixelOffset = fileHeaderSize + dibHeaderSize
    var data = Data()

    data.append(contentsOf: [0x42, 0x4d])
    data.append(contentsOf: uint32LE(UInt32(pixelOffset + pixelBytes)))
    data.append(contentsOf: uint16LE(0))
    data.append(contentsOf: uint16LE(0))
    data.append(contentsOf: uint32LE(UInt32(pixelOffset)))
    data.append(contentsOf: uint32LE(UInt32(dibHeaderSize)))
    data.append(contentsOf: int32LE(Int32(width)))
    data.append(contentsOf: int32LE(-Int32(height)))
    data.append(contentsOf: uint16LE(1))
    data.append(contentsOf: uint16LE(32))
    data.append(contentsOf: uint32LE(0))
    data.append(contentsOf: uint32LE(UInt32(pixelBytes)))
    data.append(contentsOf: int32LE(2835))
    data.append(contentsOf: int32LE(2835))
    data.append(contentsOf: uint32LE(0))
    data.append(contentsOf: uint32LE(0))

    let pixels = context.data!.assumingMemoryBound(to: UInt8.self)
    for y in 0..<height {
        for x in 0..<width {
            let offset = y * context.bytesPerRow + x * 4
            data.append(pixels[offset + 2])
            data.append(pixels[offset + 1])
            data.append(pixels[offset])
            data.append(pixels[offset + 3])
        }
    }

    try! data.write(to: url)
}

try! FileManager.default.createDirectory(at: iconsDirectory, withIntermediateDirectories: true)
try! FileManager.default.createDirectory(at: publicImagesDirectory, withIntermediateDirectories: true)
try! FileManager.default.createDirectory(at: nsisDirectory, withIntermediateDirectories: true)

let disabledContext = normalizeToBlackAlpha(loadImage(from: disabledSourceURL), canvasSize: mainIconSize)
let enabledContext = normalizeToBlackAlpha(loadImage(from: enabledSourceURL), canvasSize: mainIconSize)
let disabledIcon = image(from: disabledContext)
let enabledIcon = image(from: enabledContext)

let disabledBounds = alphaBounds(disabledContext)
let enabledBounds = alphaBounds(enabledContext)

writePNG(disabledIcon, to: iconsDirectory.appendingPathComponent("icon.png"))
writePNG(enabledIcon, to: iconsDirectory.appendingPathComponent("icon-enabled.png"))
writePNG(disabledIcon, to: publicImagesDirectory.appendingPathComponent("german-character-keys-icon.png"))

let disabledTray1x = makeTrayIcon(source: disabledIcon, cropBounds: disabledBounds, size: 18, inset: trayInset1x)
let disabledTray2x = makeTrayIcon(source: disabledIcon, cropBounds: disabledBounds, size: 36, inset: trayInset2x)
let enabledTray1x = makeTrayIcon(source: enabledIcon, cropBounds: enabledBounds, size: 18, inset: trayInset1x)
let enabledTray2x = makeTrayIcon(source: enabledIcon, cropBounds: enabledBounds, size: 36, inset: trayInset2x)

for (image, name) in [
    (disabledTray1x, "tray-macos-disabled.png"),
    (disabledTray2x, "tray-macos-disabled@2x.png"),
    (enabledTray1x, "tray-macos-enabled.png"),
    (enabledTray2x, "tray-macos-enabled@2x.png"),
] {
    validateTemplate(image, name: name)
    writePNG(image, to: iconsDirectory.appendingPathComponent(name))
}

let installerHeader = makeInstallerBitmap(width: 150, height: 57) { context in
    fill(context, x: 0, y: 0, width: 150, height: 57, red: 255, green: 255, blue: 255)
    fill(context, x: 0, y: 0, width: 6, height: 57, red: 31, green: 41, blue: 51)
    fill(context, x: 6, y: 55, width: 144, height: 2, red: 228, green: 231, blue: 236)
    context.draw(disabledIcon, in: CGRect(x: 16, y: 8, width: 41, height: 41))
}
let installerSidebar = makeInstallerBitmap(width: 164, height: 314) { context in
    fill(context, x: 0, y: 0, width: 164, height: 314, red: 245, green: 246, blue: 247)
    fill(context, x: 0, y: 0, width: 9, height: 314, red: 31, green: 41, blue: 51)
    fill(context, x: 9, y: 296, width: 155, height: 9, red: 184, green: 68, blue: 62)
    fill(context, x: 9, y: 305, width: 155, height: 9, red: 240, green: 180, blue: 41)
    context.draw(disabledIcon, in: CGRect(x: 18, y: 83, width: 128, height: 128))
}
writeBMP(installerHeader, to: nsisDirectory.appendingPathComponent("installer-header.bmp"))
writeBMP(installerSidebar, to: nsisDirectory.appendingPathComponent("installer-sidebar.bmp"))

print("disabled bounds: \(disabledBounds)")
print("enabled bounds: \(enabledBounds)")
print("disabled tray crop: \(disabledBounds)")
print("enabled tray crop: \(enabledBounds)")
print("generated app, tray, public, and NSIS bitmap icons")
