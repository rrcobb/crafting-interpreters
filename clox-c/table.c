#include <stdlib.h>
#include <string.h>

#include "memory.h"
#include "object.h"
#include "table.h"
#include "value.h"

#define TABLE_MAX_LOAD 0.75
#define SMALL_TABLE_THRESHOLD 8

#ifdef DEBUG_TRACK_TABLE
#include <stdio.h>

static size_t total_probes = 0;
static size_t total_lookups = 0;
static size_t small_lookups = 0;
static size_t max_probes = 0;
static size_t empty_hits = 0;
static size_t small_hits = 0;
static size_t tombstone_hits = 0;
static size_t key_hits = 0;
static size_t total_table_size = 0;
static size_t total_table_count = 0;
#endif


void initTable(Table* table) {
  table->count = 0;
  table->capacity = 0;
  table->entries = NULL;

#ifdef DEBUG_TRACK_TABLE
  total_table_count++;
#endif
}

void freeTable(Table* table) {
  FREE_ARRAY(Entry, table->entries, table->capacity);
  initTable(table);
}

#ifdef DEBUG_TRACK_TABLE
void printTableStats() {
  printf("Table Statistics:\n");
  printf("Total lookups: %zu\n", total_lookups);
  printf("Total probes: %zu\n", total_probes);
  printf("Average probes per lookup: %.2f\n", (double)total_probes / total_lookups);
  printf("Max probes in a single lookup: %zu\n", max_probes);
  printf("Small table lookups: %zu\n", small_lookups);
  printf("Small table hits: %zu\n", small_hits);
  printf("Empty slot hits: %zu\n", empty_hits);
  printf("Tombstone hits: %zu\n", tombstone_hits);
  printf("Key hits: %zu\n", key_hits);
  
  // If tracking table sizes
  if (total_table_count > 0) {
    printf("Average table size: %.2f\n", (double)total_table_size / total_table_count);
  }
}
#endif

static Entry* findEntry(Entry* entries, int capacity, ObjString* key) {

  #ifdef DEBUG_TRACK_TABLE
  size_t probes = 0;
  total_lookups++;
  #endif

  if (capacity <= SMALL_TABLE_THRESHOLD) {
    #ifdef DEBUG_TRACK_TABLE
    small_lookups++;
    #endif
    // Linear search for small tables
    for (int i = 0; i < capacity; i++) {
      Entry* entry = &entries[i];
      #ifdef DEBUG_TRACK_TABLE
      total_probes++;
      #endif
      if (entry->key == key) {
        #ifdef DEBUG_TRACK_TABLE
        small_hits++;
        #endif
        return entry;
      }
    }
    // If we didn't find the key, fall through to regular probing
  }

  Entry* tombstone = NULL;
  uint32_t index = key->hash & (capacity - 1);

  for (;;) {
    Entry* entry = &entries[index];

    #ifdef DEBUG_TRACK_TABLE
    probes++;
    #endif

    if (entry->key == key) {
      #ifdef DEBUG_TRACK_TABLE
      if (probes > max_probes) max_probes = probes;
      total_probes += probes;
      key_hits++;
      #endif
      // We found the key.
      return entry;
    }

    if (entry->key == NULL) {
      if (IS_NIL(entry->value)) {
        #ifdef DEBUG_TRACK_TABLE
        empty_hits++;
        if (probes > max_probes) max_probes = probes;
        total_probes += probes;
        #endif
        // Empty entry.
        return tombstone != NULL ? tombstone : entry;
      } else {
        #ifdef DEBUG_TRACK_TABLE
        tombstone_hits++;
        #endif
        // We found a tombstone.
        if (tombstone == NULL) tombstone = entry;
      }
    }

    index = (index + 1) & (capacity - 1);
  }
#ifdef DEBUG_TRACK_TABLE
  if (probes > max_probes) max_probes = probes;
  total_probes += probes;
#endif
}

bool tableGet(Table* table, ObjString* key, Value* value) {
  if (table->count == 0) return false;

  Entry* entry = findEntry(table->entries, table->capacity, key);
  if (entry->key == NULL) return false;

  *value = entry->value;
  return true;
}

bool tableDelete(Table* table, ObjString* key) {
  if (table->count == 0) return false;

  // Find the entry.
  Entry* entry = findEntry(table->entries, table->capacity, key);
  if (entry->key == NULL) return false;

  // Place a tombstone in the entry.
  entry->key = NULL;
  entry->value = BOOL_VAL(true);

  return true;
}

static void adjustCapacity(Table* table, int capacity) {
  Entry* entries = ALLOCATE(Entry, capacity);
  for (int i = 0; i < capacity; i++) {
    entries[i].key = NULL;
    entries[i].value = NIL_VAL;
  }
  
#ifdef DEBUG_TRACK_TABLE
  total_table_size -= table->capacity;
  total_table_size += capacity;
#endif

  table->count = 0;
  for (int i = 0; i < table->capacity; i++) {
    Entry* entry = &table->entries[i];
    if (entry->key == NULL) continue;

    Entry* dest = findEntry(entries, capacity, entry->key);
    dest->key = entry->key;
    dest->value = entry->value;
    table->count++;
  }

  FREE_ARRAY(Entry, table->entries, table->capacity);
  table->entries = entries;
  table->capacity = capacity;
}

bool tableSet(Table* table, ObjString* key, Value value) {
  if (table->count + 1 > table->capacity * TABLE_MAX_LOAD) {
    int capacity = GROW_CAPACITY(table->capacity);
    adjustCapacity(table, capacity);
  }
  Entry* entry = findEntry(table->entries, table->capacity, key);

  bool isNewKey = entry->key == NULL;
  if (isNewKey && IS_NIL(entry->value)) table->count++;

  entry->key = key;
  entry->value = value;
  return isNewKey;
}

void tableAddAll(Table* from, Table* to) {
  for (int i = 0; i < from->capacity; i++) {
    Entry* entry = &from->entries[i];
    if (entry->key != NULL) {
      tableSet(to, entry->key, entry->value);
    }
  }
}

ObjString* tableFindString(Table* table, const char* chars,
                           int length, uint32_t hash) {
  if (table->count == 0) return NULL;

  uint32_t index = hash & (table->capacity - 1);

  for (;;) {
    Entry* entry = &table->entries[index];

    if (entry->key == NULL) {
      // Stop if we find an empty non-tombstone entry.
      if (IS_NIL(entry->value)) return NULL;
    } else if (entry->key->length == length &&
        entry->key->hash == hash &&
        memcmp(entry->key->chars, chars, length) == 0) {
      // We found it.
      return entry->key;
    }

    index = (index + 1) & (table->capacity - 1);
  }
}

void tableRemoveWhite(Table* table) {
  for (int i = 0; i < table->capacity; i++) {
    Entry* entry = &table->entries[i];
    if (entry->key != NULL && !entry->key->obj.isMarked) {
      tableDelete(table, entry->key);
    }
  }
}

void markTable(Table* table) {
  for (int i = 0; i < table->capacity; i++) {
    Entry* entry = &table->entries[i];
    markObject((Obj*)entry->key);
    markValue(entry->value);
  }
}
