import SwiftUI

@main
struct XcodePilotDemoApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}

struct ContentView: View {
    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "airplane.circle.fill")
                .font(.system(size: 64))
                .foregroundStyle(.blue)
            Text("Xcode Pilot Demo")
                .font(.title.bold())
            Text("Built & run from Zed")
                .foregroundStyle(.secondary)
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
