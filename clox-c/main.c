#include "common.h"

#include "chunk.h"
#include "debug.h"
#include "vm.h"

int main(int argc, const char* argv[]) {
  initVM();

  // Chunk 1: -((1.2 + 3.4) / 5.6)
  Chunk chunk;
  initChunk(&chunk);

  writeConstant(&chunk, 1.2, 1);
  writeConstant(&chunk, 3.4, 1);

  writeChunk(&chunk, OP_ADD, 1);

  writeConstant(&chunk, 5.6, 1);

  writeChunk(&chunk, OP_DIVIDE, 1);
  writeChunk(&chunk, OP_NEGATE, 1);
  writeChunk(&chunk, OP_RETURN, 1);

  disassembleChunk(&chunk, "test chunk");
  interpret(&chunk);
  initVM();
  freeChunk(&chunk);

  // Chunk 2: 1 * 2 + 3
  Chunk chunk_2;
  initChunk(&chunk_2);
  
  writeConstant(&chunk_2, 1.0, 2);
  writeConstant(&chunk_2, 2.0, 2);
  writeChunk(&chunk_2, OP_MULTIPLY, 2);

  writeConstant(&chunk_2, 3.0, 2);
  writeChunk(&chunk_2, OP_ADD, 2);
  writeChunk(&chunk_2, OP_RETURN, 2);

  disassembleChunk(&chunk_2, "1 * 2 + 3");
  interpret(&chunk_2);
  initVM();

  // Chunk 3: 1 + 2 * 3
  Chunk chunk_3;
  initChunk(&chunk_3);

  writeConstant(&chunk_3, 1.0, 3);
  writeConstant(&chunk_3, 2.0, 3);
  writeConstant(&chunk_3, 3.0, 3);
  writeChunk(&chunk_3, OP_MULTIPLY, 3);
  writeChunk(&chunk_3, OP_ADD, 3);
  writeChunk(&chunk_3, OP_RETURN, 3);

  disassembleChunk(&chunk_3, "1 + 2 * 3");
  interpret(&chunk_3);
  initVM();

  // Chunk 4: 3 - 2 - 1
  Chunk chunk_4;
  initChunk(&chunk_4);

  writeConstant(&chunk_4, 3.0, 4);
  writeConstant(&chunk_4, 2.0, 4);
  writeChunk(&chunk_4, OP_SUBTRACT, 4);
  writeConstant(&chunk_4, 1.0, 4);
  writeChunk(&chunk_4, OP_SUBTRACT, 4);
  writeChunk(&chunk_4, OP_RETURN, 4);

  disassembleChunk(&chunk_4, "3 - 2 - 1");
  interpret(&chunk_4);
  initVM();

  // Chunk 5: 1 + 2 * 3 - 4 / -5
  Chunk chunk_5;
  initChunk(&chunk_5);

  writeConstant(&chunk_5, 1.0, 5);
  writeConstant(&chunk_5, 2.0, 5);
  writeConstant(&chunk_5, 3.0, 5);
  writeChunk(&chunk_5, OP_MULTIPLY, 5);
  writeChunk(&chunk_5, OP_ADD, 5);
  writeConstant(&chunk_5, 4.0, 5);
  writeConstant(&chunk_5, 5.0, 5);
  writeChunk(&chunk_5, OP_NEGATE, 5);
  writeChunk(&chunk_5, OP_DIVIDE, 5);
  writeChunk(&chunk_5, OP_SUBTRACT, 5);
  writeChunk(&chunk_5, OP_RETURN, 5);

  disassembleChunk(&chunk_5, "1 + 2 * 3 - 4 / -5");
  interpret(&chunk_5);
  initVM();

  freeVM();
  freeChunk(&chunk_2); 
  freeChunk(&chunk_3); 
  freeChunk(&chunk_4); 
  freeChunk(&chunk_5); 
  return 0;
}
