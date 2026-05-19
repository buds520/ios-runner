import os
import SwiftUI

private let logger = Logger(subsystem: "com.xcodepilot.demo", category: "XcodePilotDemo")

@main
struct XcodePilotDemoApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}

struct ContentView: View {
    @State private var tapCount = 0
    @State private var lastMessage = "尚未点击"

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "iphone.gen3")
                .font(.system(size: 56))
                .foregroundStyle(.blue)

            Text("iOS-Runner Demo")
                .font(.title.bold())

            Text("在 Zed 终端跑 Run，点按钮看日志")
                .font(.subheadline)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)

            Button {
                tapCount += 1
                let message = logTap(count: tapCount)
                lastMessage = message
            } label: {
                Label("点我打印日志", systemImage: "hand.tap.fill")
                    .font(.headline)
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 14)
            }
            .buttonStyle(.borderedProminent)

            VStack(alignment: .leading, spacing: 8) {
                Text("点击次数：\(tapCount)")
                Text("最近日志：")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                Text(lastMessage)
                    .font(.caption.monospaced())
                    .frame(maxWidth: .infinity, alignment: .leading)
            }
            .padding()
            .background(.quaternary.opacity(0.5), in: RoundedRectangle(cornerRadius: 12))
        }
        .padding(24)
    }

    /// stdout → Zed 终端（simctl --console-pty）
    private func logTap(count: Int) -> String {
        let formatter = DateFormatter()
        formatter.dateFormat = "HH:mm:ss"
        let time = formatter.string(from: Date())
        let message = "📱 tap #\(count)  \(time)"

        print(message)
        logger.info("\(message, privacy: .public)")

        return message
    }
}

#Preview {
    ContentView()
}
