import SwiftUI
import SwiftyJSON

@main
struct CocoaPodsDemoApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}

struct ContentView: View {
    @State private var tapCount = 0
    @State private var podMessage = "CocoaPods 未测试"

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "shippingbox.fill")
                .font(.system(size: 56))
                .foregroundStyle(.orange)

            Text("CocoaPods Demo")
                .font(.title.bold())

            Text("iOS-Runner · Podfile + workspace")
                .font(.subheadline)
                .foregroundStyle(.secondary)

            Text(podMessage)
                .font(.caption.monospaced())
                .multilineTextAlignment(.center)
                .padding()
                .frame(maxWidth: .infinity)
                .background(.quaternary.opacity(0.5), in: RoundedRectangle(cornerRadius: 12))

            Button {
                tapCount += 1
                let json = JSON([
                    "source": "CocoaPodsDemo",
                    "tap": tapCount,
                    "pod": "SwiftyJSON",
                ])
                podMessage = json.rawString() ?? json.description
                print("📦 \(podMessage)")
            } label: {
                Label("点我（Pod + 日志）", systemImage: "hand.tap.fill")
                    .font(.headline)
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 14)
            }
            .buttonStyle(.borderedProminent)
            .tint(.orange)
        }
        .padding(24)
        .onAppear {
            let json = JSON(["pods": "SwiftyJSON loaded"])
            podMessage = json["pods"].stringValue
        }
    }
}

#Preview {
    ContentView()
}
