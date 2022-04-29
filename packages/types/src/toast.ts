/*
 * Copyright 2020 Adobe. All rights reserved.
 * This file is licensed to you under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License. You may obtain a copy
 * of the License at http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under
 * the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR REPRESENTATIONS
 * OF ANY KIND, either express or implied. See the License for the specific language
 * governing permissions and limitations under the License.
 */

import { DOMProps, StyleProps } from "./shared";
import { JSX } from "solid-js";

export interface ToastOptions extends DOMProps, StyleProps {
  actionLabel?: JSX.Element;
  onAction?: (...args: unknown[]) => void;
  shouldCloseOnAction?: boolean;
  onClose?: (...args: unknown[]) => void;
  timeout?: number;
}

export interface ToastProps extends ToastOptions {
  children?: JSX.Element;
  variant?: "positive" | "negative" | "info";
  toastKey?: string;
  timer?: unknown;
}

export interface ToastState {
  onAdd?: (content: JSX.Element, options: ToastProps) => void;
  onRemove?: (idKey: string) => void;
  setToasts?: (value: ToastStateValue[]) => void;
  toasts?: ToastStateValue[];
}

export interface ToastStateValue {
  content: JSX.Element;
  props: ToastProps;
  timer: unknown;
}
