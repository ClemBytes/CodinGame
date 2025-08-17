# Parse input
nb_input = int(input())
nb_output = int(input())
inputs = dict()
outputs = dict()
for i in range(nb_input):
    input_name, input_signal = input().split(' ')
    inputs[input_name] = input_signal
for i in range(nb_output):
    output_name, gate, input_name_1, input_name_2 = input().split(' ')
    outputs[output_name] = {
        'gate': gate,
        'input_name_1': input_name_1,
        'input_name_2': input_name_2,
    }

# Define gates
def and_gate(input1, input2):
    if len(input1) != len(input2):
        raise "AND GATE: input 1 and 2 should have the same length!"
    result = ""
    for i in range(len(input1)):
        if input1[i] == input2[i] == '-':
            result += '-'
        else:
            result += '_'
    return result

def or_gate(input1, input2):
    if len(input1) != len(input2):
        raise "AND GATE: input 1 and 2 should have the same length!"
    result = ""
    for i in range(len(input1)):
        if input1[i] == '-' or input2[i] == '-':
            result += '-'
        else:
            result += '_'
    return result

def xor_gate(input1, input2):
    if len(input1) != len(input2):
        raise "AND GATE: input 1 and 2 should have the same length!"
    result = ""
    for i in range(len(input1)):
        if (input1[i] != input2[i]) and (input1[i] == '-' or input2[i] == '-'):
            result += '-'
        else:
            result += '_'
    return result

def nand_gate(input1, input2):
    if len(input1) != len(input2):
        raise "AND GATE: input 1 and 2 should have the same length!"
    result = ""
    for i in range(len(input1)):
        if input1[i] == input2[i] == '-':
            result += '_'
        else:
            result += '-'
    return result

def nor_gate(input1, input2):
    if len(input1) != len(input2):
        raise "AND GATE: input 1 and 2 should have the same length!"
    result = ""
    for i in range(len(input1)):
        if input1[i] == '-' or input2[i] == '-':
            result += '_'
        else:
            result += '-'
    return result

def nxor_gate(input1, input2):
    if len(input1) != len(input2):
        raise "AND GATE: input 1 and 2 should have the same length!"
    result = ""
    for i in range(len(input1)):
        if (input1[i] != input2[i]) and (input1[i] == '-' or input2[i] == '-'):
            result += '_'
        else:
            result += '-'
    return result

# Then apply them
for output_name in outputs:
    output_infos = outputs[output_name]
    result = output_name + ' '
    if output_infos['gate'] == 'AND':
        result += and_gate(inputs[output_infos['input_name_1']], inputs[output_infos['input_name_2']])
    elif output_infos['gate'] == 'OR':
        result += or_gate(inputs[output_infos['input_name_1']], inputs[output_infos['input_name_2']])
    elif output_infos['gate'] == 'XOR':
        result += xor_gate(inputs[output_infos['input_name_1']], inputs[output_infos['input_name_2']])
    elif output_infos['gate'] == 'NAND':
        result += nand_gate(inputs[output_infos['input_name_1']], inputs[output_infos['input_name_2']])
    elif output_infos['gate'] == 'NOR':
        result += nor_gate(inputs[output_infos['input_name_1']], inputs[output_infos['input_name_2']])
    elif output_infos['gate'] == 'NXOR':
        result += nxor_gate(inputs[output_infos['input_name_1']], inputs[output_infos['input_name_2']])
    print(result)
