/**
 * @license
 * Copyright 2023 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */

//basic index provided as a template by google
//it manages the UI 

import * as Blockly from 'blockly';

import { blocks } from './blocks/json';

import { jsonGenerator } from './generators/json';

import { save, load } from './serialization';
import { toolbox } from './toolbox';
import './index.css';

import './renderers/renderer.js';

// Register the blocks and generator with Blockly
Blockly.common.defineBlocks(blocks);


// Set up UI elements and inject Blockly
const codeDiv = document.getElementById('generatedCode').firstChild;

const blocklyDiv = document.getElementById('blocklyDiv');
const ws = Blockly.inject(blocklyDiv, {
  renderer: 'custom_renderer',
  toolbox,
});

// This function resets the code div and shows the
// generated code from the workspace.
const runCode = () => {

  const code = jsonGenerator.workspaceToCode(ws);
  codeDiv.innerText = code;
};

// Load the initial state from storage and run the code.
// this messes up code generation, so I keep it commented out
//load(ws)
//runCode();

// Every time the workspace changes state, save the changes to storage.
ws.addChangeListener((e) => {
  // UI events are things like scrolling, zooming, etc.
  // No need to save after one of these.
  if (e.isUiEvent) return;
  //save(ws);
});


// Whenever the workspace changes meaningfully, run the code again.
ws.addChangeListener((e) => {
  // Don't run the code when the workspace finishes loading; we're
  // already running it once when the application starts.
  // Don't run the code during drags; we might have invalid state.
  if (e.isUiEvent || e.type == Blockly.Events.FINISHED_LOADING ||
    ws.isDragging()) {
    return;
  }

  runCode();
});

