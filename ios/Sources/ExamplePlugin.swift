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
    wKWebView.scrollView.bounces = false
  }
}

@_cdecl("init_plugin_ios_no_bounce")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
