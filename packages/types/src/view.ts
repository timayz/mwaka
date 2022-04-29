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

import { DOMProps, StyleProps, ViewStyleProps } from "./shared";
import { JSX, Component } from "solid-js";

export interface ContentProps extends DOMProps, StyleProps {
  /**
   * Content content.
   */
  children: JSX.Element;
}

export interface FooterProps extends DOMProps, StyleProps {
  /**
   * Footer content.
   */
  children: JSX.Element;
}

export interface HeaderProps extends DOMProps, StyleProps {
  /**
   * Header content.
   */
  children: JSX.Element;
}

export interface ViewProps extends ViewStyleProps, DOMProps {
  /**
   * The element to render as the node.
   */
  elementType?: string | Component<unknown>;
  /**
   * Children to be displayed in the View.
   */
  children?: JSX.Element;
}
