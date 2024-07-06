import type { NetworkConfig, NetworkInstanceInfo } from 'type.mjs';
export declare function parseNetworkConfig(cfg: NetworkConfig): Promise<string>;
export declare function startNetworkInstance(cfg: NetworkConfig): Promise<null>;
export declare function stopNetworkInstance(id: string, emitTag?: string): Promise<null>;
export declare function collectNetworkInfos(): Promise<Map<string, NetworkInstanceInfo>>;
