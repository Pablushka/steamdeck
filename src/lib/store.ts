import { writable, get } from "svelte/store";
import defaultSchema from "./deck_config.json";

export interface ButtonConfig {
  id: string;
  label: string;
  icon: string;
  type: "launch" | "hotkey" | "folder" | "back" | "media" | "sys_control";
  payload: string;
  bgColor?: string;
  glowColor?: string;
}

export interface DeckPage {
  pageIndex: number;
  buttons: ButtonConfig[];
}

export interface ProfileConfig {
  name: string;
  isFolder?: boolean;
  parentProfile?: string;
  pages: DeckPage[];
}

export interface DeckConfig {
  activeProfile: string;
  profiles: Record<string, ProfileConfig>;
}

// Stores
export const deckConfig = writable<DeckConfig>(defaultSchema as any);
export const currentProfileName = writable<string>("Default");
export const currentPageIndex = writable<number>(0);
export const systemStats = writable({ cpu: 0, ram: 0, gpu: 0, netDown: 0, netUp: 0 });
export const mediaState = writable({ volume: 50, brightness: 70, playing: false, track: "Spotify Active" });

// Navigation history stack for folders
export const navigationStack = writable<string[]>([]);

/**
 * Navigate down into a nested profile folder
 */
export function navigateToFolder(folderName: string) {
  const stack = get(navigationStack);
  const current = get(currentProfileName);
  
  navigationStack.set([...stack, current]);
  currentProfileName.set(folderName);
  currentPageIndex.set(0);
}

/**
 * Navigate backward up the folder stack
 */
export function navigateBack() {
  const stack = get(navigationStack);
  if (stack.length > 0) {
    const prev = stack[stack.length - 1];
    navigationStack.set(stack.slice(0, -1));
    currentProfileName.set(prev);
    currentPageIndex.set(0);
  } else {
    currentProfileName.set("Default");
    currentPageIndex.set(0);
  }
}
