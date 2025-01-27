#include <vector>
#include <iostream>
#include <cstddef>

class BTNode{
    std::vector<int> keys;
    std::vector<BTNode*> children; // C
    bool isLeaf;
    int d; 
    int numberOfKeys;

    public:
        BTNode(int d, bool isLeaf);

        void insertNonFull(int k);

        void splitChild(int i, BTNode *n);

        void traverse();

        BTNode *search(int k);

    friend class BTree;
};

BTNode::BTNode(int _d, bool _isLeaf){
    d = _d;
    isLeaf = _isLeaf;
    numberOfKeys = 0;
}

void BTNode::traverse(){
    for(int ii = 0; ii < numberOfKeys; ii++){
        if(!isLeaf){
            (children[ii])->traverse();
        }
        std::cout << " " << keys[ii];
    }
}

BTNode *BTNode::search(int k){
    int ii = 0;
    while(ii < numberOfKeys && k > keys[ii]){
        ii++;
    }

    if(ii < numberOfKeys && keys[ii] == k){
        return this;
    }

    if(isLeaf){
        return nullptr;
    }

    return children[ii]->search(k);
}

// índice do filho i a ser dividido e o nó pai
void BTNode::splitChild(int i, BTNode *child){
    BTNode *z = new BTNode(child->d, child->isLeaf);
    z->numberOfKeys = d-1;

    //copiar as ultimas d chaves de i para o novo nó z
    for(int j = 0; j < d-1; j++){
        z->keys[j] = child->keys[j+(d+1)];
    }

    //copiar os últimos d+1 filhos de i para o novo z
    if(child->isLeaf == false){
        for(int j = 0; j < d; j++){
            z->children[j] = child->children[j+d];
        }
    }

    child->numberOfKeys = d-1;

    for (int j = numberOfKeys; j >= i + 1; j--) {
        children[j + 1] = children[j];
    }

    //inserir o z como nó do pai
    children[i+1] = z;

     // Mover as chaves do nó pai para abrir espaço para a chave promovida
    for (int j = numberOfKeys - 1; j >= i; j--) {
        keys[j + 1] = keys[j];
    }

    //promover a chave do meio do filho i para o nó pai
    keys[i] = child->keys[d-1];
    numberOfKeys++;
}

void BTNode::insertNonFull(int k){
    int i = numberOfKeys - 1;

    if(isLeaf){
        while(i >= 0 && keys[i] > k){
            keys[i+1] = keys[i];
            i--;
        }

        keys[i+1] = k;
        numberOfKeys++;
    }else{
        // Find the child which is going to have the new key
        while (i >= 0 && keys[i] > k)
            i--;

        // See if the found child is full
        if (children[i+1]->numberOfKeys == 2*d){
            // If the child is full, then split it
            splitChild(i+1, children[i+1]);

            // After split, the middle key of C[i] goes up and
            // C[i] is splitted into two.  See which of the two
            // is going to have the new key
            if (keys[i+1] < k)
                i++;
        }
        children[i+1]->insertNonFull(k);
    }
}

class BTree{
    BTNode *root;
    int d;

    public:
        BTree(int _d){
            root = nullptr;
            d = _d;
        }

        void traverse(){
            if(root != nullptr){
                root->traverse();
            }
        }

        BTNode* search(int k){
            return (root = nullptr)? nullptr : root->search(k);
        }

        void insert(int k);
};

void BTree::insert(int k){
    if (root == nullptr){
        // Allocate memory for root
        root = new BTNode(k, true);
        root->keys[0] = k;  // Insert key
        root->numberOfKeys = 1;
    }else{
        if(root->numberOfKeys == 2*d){
            BTNode *s = new BTNode(d, false);

            s->children[0] = root;

            s->splitChild(0, root);

            int i = 0;
            if(s->keys[0] < k){
                i++;
            }

            s->children[i]->insertNonFull(k);

            root = s;

        }else{
            root->insertNonFull(k);
        }
    }
}



int main(){
    BTree t(3); // A B-Tree with minimum degree 3
    t.insert(10);
    t.insert(20);
    t.insert(5);
    t.insert(6);
    t.insert(12);
    t.insert(30);
    t.insert(7);
    t.insert(17);

    std::cout << "Traversal of the constructed tree is ";
    t.traverse();

    int k = 6;
    (t.search(k) != NULL)? std::cout << "\nPresent" : std::cout << "\nNot Present";

    k = 15;
    (t.search(k) != NULL)? std::cout << "\nPresent" : std::cout << "\nNot Present";

    return 0;

}





//          6 7 8
         ///
// 1 2 3 4 <- 5
// 1 2 3 4 5
