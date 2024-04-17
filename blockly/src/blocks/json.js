import * as Blockly from 'blockly';

import { ColorWheelField } from "blockly-field-color-wheel";
import { FieldSlider } from "@blockly/field-slider";


// Custom validator to ensure VALUE1 <= VALUE2
// function validateSliderValues(newValue) {
//   var value1 = parseFloat(this.getSourceBlock().getFieldValue('MIN_ALPHA'));
//   var value2 = parseFloat(this.getSourceBlock().getFieldValue('MAX_ALPHA'));

//   // Ensure VALUE2 is not less than VALUE1
//   if (this.name === 'MAX_ALPHA' && newValue < value1) {
//     return value1;
//   }

//   return newValue;
// }

Blockly.Blocks['particle_base'] = {
  init: function () {
    const validator_max = function (newValue) {

      var value_min = parseFloat(this.getSourceBlock().getFieldValue('MIN_ALPHA'));
      if (newValue < value_min)
        return value_min;
      return newValue;
    };
    const validator_min = function (newValue) {
      var value_max = parseFloat(this.getSourceBlock().getFieldValue('MAX_ALPHA'));
      if (newValue > value_max)
        return value_max;
      return newValue;
    };

    this.appendDummyInput()
      .appendField("Name: ")
      .appendField(new Blockly.FieldTextInput(""), "NAME")
      .appendField(" Colour:")
      .appendField(new ColorWheelField("#7fdab4", 150, {
        layoutDirection: "vertical",
      }),
        "COLOR")
      .appendField(" Alpha: ")
      .appendField(new FieldSlider(1, 0, 1, 0.1, validator_min), "MIN_ALPHA")
      .appendField(new FieldSlider(1, 0, 1, 0.1, validator_max), "MAX_ALPHA");


    this.setNextStatement(true, null);
    this.setInputsInline(true);
    this.setDeletable(false);
    this.setMovable(true);

    this.setColour(160);

  }

};



Blockly.Blocks['custom_input_color'] = {
  init: function () {
    this.appendDummyInput()

      .appendField(new ColorWheelField("#7fdab4", 150, {
        layoutDirection: "vertical",
      }),
        "COLOR")


    this.setOutput(true, null);
    this.setColour(160);

  }
};

Blockly.Blocks['is_equal'] = {

}

Blockly.Blocks['test_field_slider'] = {
  init: function () {
    this.appendDummyInput()
      .appendField('slider: ')
      .appendField(new FieldSlider(50), 'FIELDNAME');
  },
};

Blockly.defineBlocksWithJsonArray([
  {
    type: "if",
    message0: "if %1 %2 %3",
    args0: [
      {
        type: "input_dummy",
      },
      {
        type: "input_value",
        name: "CONDITION",
        check: "Boolean",
      },
      {
        type: "input_statement",
        name: "THEN",
      },
    ],
    inputsInline: true,
    previousStatement: null,
    nextStatement: null,
    colour: 330,

  },
]);

export const blocks = Blockly.common.createBlockDefinitionsFromJsonArray([

  {
    "type": "my_block",
    "message0": "My Block %1 text %2",
    "args0": [
      {
        "type": "input_dummy"
      },
      {
        "type": "boolean",
        "name": "TEXT",
        "check": "String"
      }
    ],
    "inputsInline": true,
    "output": "String",
    "colour": 230,
    "tooltip": "Tooltip text",
    "helpUrl": "https://www.example.com/"
  },

  //#region tutorial
  {
    "type": "object",
    "message0": "{ %1 %2 }",
    "args0": [
      {
        "type": "input_dummy"
      },
      {
        "type": "input_statement",
        "name": "MEMBERS"
      }
    ],
    "output": null,
    "colour": 230,
  },
  {
    "type": "member",
    "message0": "%1 %2 %3",
    "args0": [
      {
        "type": "field_input",
        "name": "MEMBER_NAME",
        "text": ""
      },
      {
        "type": "field_label",
        "name": "COLON",
        "text": ":"
      },
      {
        "type": "input_value",
        "name": "MEMBER_VALUE"
      }
    ],
    "previousStatement": null,
    "nextStatement": null,
    "colour": 230,
  }
  //#endregion

]
);


