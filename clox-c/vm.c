#include <stdio.h>

#include "common.h"
#include "vm.h"
#include "compiler.h"
#include "debug.h"

VM vm;

static void resetStack() {
  vm.stackTop = vm.stack;
}

void push(Value value) {
  *vm.stackTop = value;
  vm.stackTop++;
}

Value pop() {
  vm.stackTop--;
  return *vm.stackTop;
}

void initVM() {
  resetStack();
}

void freeVM() {
}

// note that given the time spent in this loop, optimization techniques are worth it, well-studied, and abundant. Look up: direct threaded code, jump table, computed goto
static InterpretResult run() {
#define READ_BYTE() *(vm.ip++)
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])

#define BINARY_OP(op) \
  do { \
    double b = pop(); \
    double a = pop(); \
    push(a op b); \
  } while (false)

  for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
    printf("          ");
    for (Value* slot = vm.stack; slot < vm.stackTop; slot++) {
      printf("[ ");
      printValue(*slot);
      printf(" ]");
    }
    printf("\n");
    disassembleInstruction(vm.chunk, (int)(vm.ip - vm.chunk->code));
#endif
    uint8_t instruction;
    switch(instruction = READ_BYTE()) {
      // todo op_constant_long
      case OP_CONSTANT: {
                          Value constant = READ_CONSTANT();
                          push(constant);
                          break;
                        }
      case OP_ADD:      BINARY_OP(+); break;
      case OP_SUBTRACT: BINARY_OP(-); break;
      case OP_MULTIPLY: BINARY_OP(*); break;
      case OP_DIVIDE:   BINARY_OP(/); break;
      case OP_NEGATE:   *(vm.stackTop - 1) = -*(vm.stackTop - 1); break;
      case OP_RETURN: {
                        printValue(pop());
                        printf("\n");
                        return INTERPRET_OK;
                      }
    }
  }

#undef READ_BYTE
#undef READ_CONSTANT
#undef BINARY_OP
}

InterpretResult interpret(const char* source) {
  compile(source);
  return INTERPRET_OK;
}

