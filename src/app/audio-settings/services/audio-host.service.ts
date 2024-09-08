import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";

export enum WindowsHost {
  ASIO = "ASIO",
  WASAPI = "WASAPI",
}

@Injectable({
    providedIn: 'root',
})
export class AudioHostService {
    constructor() {}

    getCurrentHost(): Promise<WindowsHost> {
        return invoke('get_current_host');
    }

    switchHost(host: WindowsHost): Promise<void> {
        return invoke('switch_host', { host });
    }
}
