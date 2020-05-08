// To get some practice with pointers, define a doubly-linked list of heap-allocated strings. 
// Write functions to insert, find, and delete items from it.
// Test them. 
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct Node {
  char* text;
  struct Node* prev;
  struct Node* next;
} Node;

typedef struct List {
  struct Node* first;
  struct Node* last;
} List;

// insert
void insert(struct Node *node, struct Node *addBefore) {
  // insert before a particular other node
  node->next = addBefore;
  if (addBefore->prev) {
    struct Node *prev = addBefore->prev;
    node->prev = prev;
    prev->next = node;
  }
  addBefore->prev = node;
}

// find
struct Node* find(struct List *list, char *text_to_match) {
  // struct List list
  // iterate through the list from the front
  // and just, check each one
  struct Node *curr = list->first;
  while (curr) {
    if (strcmp(curr->text, text_to_match) == 0) {
      return curr;
    }
    curr = curr->next;
  }
  return 0;
}

// delete
void delete(struct Node *node) {
  // does not work for lists, really...
  if (node->next) {
    node->next->prev = node->prev;
  }
  if (node->prev) {
    node->prev->next = node->next;
  }
}

void p(char* str) {
  printf("%s", str);
}

int main() {
  printf("\ntesting doubly linked lists\n");
  char *la = (char *) malloc(2*sizeof(char));
  strcpy(la, "a");
  Node a = { la };
  Node b = { "b" };
  Node c = { "c" };
  p(a.text);
  p(b.text);
  p(c.text);
  printf("\n");

  // testing insert, a little
  insert(&a, &b);
  insert(&b, &c);
  // forward
  p(a.next->next->text);
  p(b.next->text);
  p(c.text);
  printf("\n");
  // backward
  p(c.prev->prev->text);
  p(b.prev->text);
  p(a.text);
  p("\n");

  // testing find, some
  List letters = { &a, &c };
  struct Node *found = find(&letters, "a");
  p(found->text);
  found = find(&letters, "b");
  p(found->text);
  found = find(&letters, "c");
  p(found->text);

  printf("\n");

  // smol tests for delete
  delete(&b);
  p(letters.first->text);
  p(letters.first->next->text);
  printf("\n");
  delete(&c);
  p(letters.first->text);
  delete(&a);

  printf("\n");
}
