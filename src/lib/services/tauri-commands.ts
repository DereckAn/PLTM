import type { Hint, UIElement } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

export class TauriCommands {
    static async scanElements(): Promise<UIElement[]> {
        return invoke('scan_elements');
    }

    static async checkPermissions(): Promise<boolean> {
        return invoke('check_permissions');
    }

    static async requestPermissions(): Promise<void>{
        return invoke('request_permissions');
    }

    static async openAccessibilitySettings(): Promise<void> {
        return invoke('open_accessibility_settings');
    }

    static async registerHotkey(keyCombo: string): Promise<void> {
        return invoke('register_hotkey', { keyCombo });
    }
    
    static async performClick(x: number, y: number) : Promise<void> {
        return invoke('perform_click', { x, y });
    }

    static async showHints(hints: Hint[]): Promise<void> {
        return invoke('show_hints', { hints });
    }

    static async activateNavigation(): Promise<Hint[]> {
        return invoke<Hint[]>("activate_navigation");
    }

    static async deactivateNavigation(): Promise<void> {
        return invoke("deactivate_navigation");
    }
}
