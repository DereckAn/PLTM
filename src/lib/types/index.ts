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
  
export interface UIElement {
  id: string;
  role: string;
  title: string | null;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface Hint {
  label: string;
  x: number;
  y: number;
  element_id: string;
}
