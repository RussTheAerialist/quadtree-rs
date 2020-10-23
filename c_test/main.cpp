#include "../target/quadtree.h"
#include <iostream>

using namespace std;


void main() {
	using namespace quadtree;

	Quadtree* qt = quadtree_new(10., 10., 10., 10., 0.);
	if(qt == nullptr) {
		cout << "Could not allocate quadtree" << endl;
		return;
	}

	int value = 42;
	int value2 = 27;
	int value3 = -1;

	if (quadtree_insert_point(qt, 1., 1., &value)) {
		cout << "Could not insert the point" << endl;
		return;
	}

		if (quadtree_insert_point(qt, 2., 1., &value2)) {
		cout << "Could not insert the point" << endl;
		return;
	}

			if (quadtree_insert_point(qt, 1., 2., &value3)) {
		cout << "Could not insert the point" << endl;
		return;
	}

	size_t count;
	int** result = (int **)quadtree_query(qt, 1., 2., 3., &count);
	for(size_t i = 0; i < count; i++) {
		cout << i << ": " << *result[i] << endl;
	}

	quadtree_free(qt);
	qt = nullptr;
}