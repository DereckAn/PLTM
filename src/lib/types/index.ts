export interface UIElement {
  id: string;
  role: AccessibilityRole;
  position: Rect;
  title?: string;
  value?: string;
  is_focusable: boolean;
}

interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export type AccessibilityRole =
  | "button"
  | "link"
  | "textfield"
  | "checkbox"
  | "menuitem"
  | "other";

export interface Hint {
  id: string;
  label: string;
  x: number;
  y: number;
}
