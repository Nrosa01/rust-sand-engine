import * as Blockly from 'blockly';

export const jsonGenerator = new Blockly.Generator('JSON');

//precedence is irrelevant in json format

const Order = {
  ATOMIC: 0,
};

//block basic generator 
jsonGenerator.forBlock['logic_null'] = function(block) {
  return ['null', Order.ATOMIC];
};

jsonGenerator.forBlock['text'] = function(block) {
  const textValue = block.getFieldValue('TEXT');
  const code = `"${textValue}"`;
  return [code, Order.ATOMIC];
};

jsonGenerator.forBlock['math_number'] = function(block) {
  const code = String(block.getFieldValue('NUM'));
  return [code, Order.ATOMIC];
};

jsonGenerator.forBlock['logic_boolean'] = function(block) {
  const code = (block.getFieldValue('BOOL') === 'TRUE') ? 'true' : 'false';
  return [code, Order.ATOMIC];
};

//member generator
jsonGenerator.forBlock['member'] = function(block, generator) {
  
  const name = block.getFieldValue('MEMBER_NAME');
  const value = generator.valueToCode(
      block, 'MEMBER_VALUE', Order.ATOMIC);
  const code = `"${name}": ${value}`;
  return code;
};

//array generator 
//The array block uses a mutator to dynamically change the number of inputs it has.
jsonGenerator.forBlock['lists_create_with'] = function(block, generator) {
  const values = [];
  for (let i = 0; i < block.itemCount_; i++) {
    const valueCode = generator.valueToCode(block, 'ADD' + i,
        Order.ATOMIC);
    if (valueCode) {
      values.push(valueCode);
    }
  }
  const valueString = values.join(',\n');
  const indentedValueString =
      generator.prefixLines(valueString, generator.INDENT);
  const codeString = '[\n' + indentedValueString + '\n]';
  return [codeString, Order.ATOMIC];
};