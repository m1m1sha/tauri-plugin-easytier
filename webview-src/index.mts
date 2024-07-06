import { invoke } from '@tauri-apps/api/core'
import type { NetworkConfig, NetworkInstanceInfo } from 'type.mjs'

export async function parseNetworkConfig(cfg: NetworkConfig): Promise<string> {
  return await invoke('plugin:easytier|parse_network_config', { cfg })
}

export async function startNetworkInstance(cfg: NetworkConfig): Promise<null> {
  return await invoke('plugin:easytier|start_network_instance', { cfg })
}

export async function stopNetworkInstance(id: string, emitTag?: string): Promise<null> {
  return await invoke('plugin:easytier|stop_network_instance', { id, emitTag })
}

export async function collectNetworkInfos(): Promise<Map<string, NetworkInstanceInfo>> {
  return await invoke('plugin:easytier|collect_network_infos')
}
