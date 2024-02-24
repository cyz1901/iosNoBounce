import SwiftRs
import Tauri
import UIKit
import WebKit

class PingArgs: Decodable {
  let value: String?
}

class ExamplePlugin: Plugin {
  var webView: WKWebView!

  @objc public func ping(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(PingArgs.self)
    invoke.resolve(["value": args.value ?? ""])
  }

  @objc public func disableBouncing(_ invoke: Invoke) throws {
    print("Hello, world1!")
    webView.scrollView.alwaysBounceHorizontal = false
    webView.scrollView.alwaysBounceVertical = false
    webView.scrollView.bounces = false
    print("Hello, world2")
  }
}

@_cdecl("init_plugin_ios_no_bounce")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
