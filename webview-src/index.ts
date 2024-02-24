import { invoke } from '@tauri-apps/api/core'

export async function execute() {
  await invoke('plugin:ios-no-bounce|execute')
}


export async function disableBouncing() {
  await invoke('plugin:ios-no-bounce|disableBouncing')
}