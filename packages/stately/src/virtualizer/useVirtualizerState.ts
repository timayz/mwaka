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

import { Collection, Key } from "mwaka/types";
import { createSignal, onCleanup, Accessor, createEffect } from "solid-js";
import { Layout } from "./Layout";
import { Rect } from "./Rect";
import { ReusableView } from "./ReusableView";
import { Size } from "./Size";
import { Virtualizer } from "./Virtualizer";

interface VirtualizerProps<T extends object, V, W> {
  renderView(type: string, content: T): V;
  renderWrapper(
    parent: ReusableView<T, V> | null,
    reusableView: ReusableView<T, V>,
    children: ReusableView<T, V>[],
    renderChildren: (views: ReusableView<T, V>[]) => W[],
  ): W;
  layout: Layout<T>;
  collection: Collection<T>;
  onVisibleRectChange(rect: Rect): void;
  getScrollAnchor?(rect: Rect): Key;
  transitionDuration?: number;
}

export interface VirtualizerState<T extends object, V, W> {
  visibleViews: Accessor<W[]>;
  setVisibleRect: (rect: Rect) => void;
  contentSize: Accessor<Size>;
  isAnimating: Accessor<boolean>;
  virtualizer: Virtualizer<T, V, W>;
  isScrolling: Accessor<boolean>;
  startScrolling: () => void;
  endScrolling: () => void;
}

export function useVirtualizerState<T extends object, V, W>(
  opts: VirtualizerProps<T, V, W>,
): VirtualizerState<T, V, W> {
  const [visibleViews, setVisibleViews] = createSignal<W[]>([]);
  const [contentSize, setContentSize] = createSignal(new Size());
  const [isAnimating, setAnimating] = createSignal(false);
  const [isScrolling, setScrolling] = createSignal(false);
  const virtualizer = new Virtualizer<T, V, W>();

  virtualizer.delegate = {
    setVisibleViews,
    setVisibleRect(rect: Rect) {
      virtualizer.visibleRect = rect;
      opts.onVisibleRectChange(rect);
    },
    setContentSize,
    renderView: opts.renderView,
    renderWrapper: opts.renderWrapper,
    beginAnimations: () => setAnimating(true),
    endAnimations: () => setAnimating(false),
    getScrollAnchor: opts.getScrollAnchor,
  };

  virtualizer.layout = opts.layout;
  virtualizer.collection = opts.collection;
  virtualizer.transitionDuration = opts.transitionDuration || 0;

  createEffect(() => {
    virtualizer.afterRender();
  });

  // eslint-disable-next-line arrow-body-style
  onCleanup(() => {
    virtualizer.willUnmount();
  });

  return {
    virtualizer,
    visibleViews,
    setVisibleRect: (rect: Rect) => {
      virtualizer.visibleRect = rect;
    },
    contentSize,
    isAnimating,
    isScrolling,
    startScrolling: () => {
      virtualizer.startScrolling();
      setScrolling(true);
    },
    endScrolling: () => {
      virtualizer.endScrolling();
      setScrolling(false);
    },
  };
}
