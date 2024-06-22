package com.plugin.easytier

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class LinkArgs {
    var ip: String? = null
}

@TauriPlugin
class EasytierPlugin(private val activity: Activity): Plugin(activity) {
    private val linkImplements = LinkService()

    @Command
    fun fd(invoke: Invoke) {
        val args = invoke.parseArgs(LinkArgs::class.java)
        val ret = JSObject()
        ret.put("fd", linkImplements.createVpnInterface(args.ip ?: ""))
        invoke.resolve(ret)
    }
}
