#include <iostream>

typedef struct node{
    int elem;  
    struct node *next;
}Node;

Node* init(){
    Node *n = (Node*) malloc(sizeof(Node));
    return n;
}

void push_front(Node* head, int elem) {
    Node* n = (Node*) malloc(sizeof(Node));
    n->elem = elem;
    n->next = head->next;
    head->next = n;
}

void visit(Node* head) {
    for(Node* cur = head->next; cur != NULL; cur = cur->next){
        printf("element: %d\n", cur->elem);
    }
}

int main(){
    Node* head = init();
    push_front(head, 1);
    push_front(head, 2);

    visit(head);
    return 0;
}