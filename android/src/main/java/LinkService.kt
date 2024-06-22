package com.plugin.easytier

import android.content.Intent
import android.net.VpnService
import android.os.Build
import android.os.ParcelFileDescriptor


class LinkService : VpnService() {

    override fun onCreate() {
        super.onCreate()
    }

    override fun onDestroy() {
        super.onDestroy()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        return super.onStartCommand(intent, flags, startId)
    }

    public fun createVpnInterface(ip: String): ParcelFileDescriptor {
        return Builder()
            // 设置 ip
            .addAddress(ip, 32)
            // 设置 路由
            // .addRoute("0.0.0.0", 0)
            // 设置 vpn name
            .setSession("easytier-VPN")
            .setBlocking(false)
            .also {
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                    it.setMetered(false)
                }
            }
            .establish() ?: throw IllegalStateException("无法初始化vpnInterface")
    }

}