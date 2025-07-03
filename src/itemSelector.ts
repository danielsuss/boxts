import { invoke } from "@tauri-apps/api/core";

export interface ItemSelectorConfig {
  invokeFunction: string;
  errorMessage: string;
}

export interface ItemSelectorState {
  setItems: (items: string[]) => void;
  setSelectedItemIndex: (index: number) => void;
  setCommandForItems: (command: string) => void;
  setText: (text: string) => void;
}

const ITEM_COMMANDS: Record<string, ItemSelectorConfig> = {
  "start": {
    invokeFunction: "get_voices",
    errorMessage: "Error getting voices"
  },
  "changevoice": {
    invokeFunction: "get_voices", 
    errorMessage: "Error getting voices"
  },
  "outputdevice": {
    invokeFunction: "get_output_devices",
    errorMessage: "Error getting output devices"
  },
  "volume": {
    invokeFunction: "get_volume_values",
    errorMessage: "Error getting volume values"
  },
  "lostfocus": {
    invokeFunction: "get_lostfocus_options",
    errorMessage: "Error getting lost focus options"
  }
};

export const handleItemCommand = async (
  command: string,
  state: ItemSelectorState
): Promise<boolean> => {
  const config = ITEM_COMMANDS[command];
  if (!config) {
    return false;
  }

  try {
    const result = await invoke<string[]>(config.invokeFunction);
    state.setItems(result);
    state.setSelectedItemIndex(0);
    state.setCommandForItems(command);
    state.setText("");
    return true;
  } catch (error) {
    console.error(config.errorMessage, error);
    return false;
  }
};

export const isItemCommand = (command: string): boolean => {
  return command in ITEM_COMMANDS;
};