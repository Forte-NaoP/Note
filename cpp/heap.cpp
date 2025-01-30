#include <cstdlib>
#include <utility>
#include <algorithm>
#include <iostream>

using namespace std;

#define MAX_LEN 1001

template<typename T>
struct heap {
    T *v;
    int cnt;
    int max_size;
    const bool is_max;

    heap(bool is_max, int size) : is_max(is_max), max_size(size) {
        v = new T[size+1];
        cnt = 0;
    }

    ~heap() {
        delete[] v;
    }

    void clear() { 
        cnt = 0; 
    }

    T& operator[] (int index) {
        if (index < 1 || index > cnt) throw out_of_range("Index out of range");
        return v[index];
    }

    int size() {
        return cnt;
    }

    T top() {
        if (cnt < 1) throw out_of_range("heap is empty"); 
        return v[1];
    }

    bool empty() {
        return cnt == 0;
    }

    void push(const T& x) {
        if (cnt >= max_size) throw out_of_range("heap is full");
        v[++cnt] = x;

        int cur = cnt;
        int parent = cnt / 2;

        while (cur > 1 && cmp(v[cur], v[parent])) {
            swap(v[cur], v[parent]);
            cur = parent;
            parent /= 2;
        }
    }

    void push(T&& x) {
        if (cnt >= max_size) throw out_of_range("heap is full");
        v[++cnt] = move(x);

        int cur = cnt;
        int parent = cnt / 2;

        while (cur > 1 && cmp(v[cur], v[parent])) {
            swap(v[cur], v[parent]);
            cur = parent;
            parent /= 2;
        }
    }

    void pop() {
        if (cnt < 1) throw out_of_range("heap is empty"); 
        swap(v[cnt--], v[1]);
        int cur = 1, child;

        while (true) {
            child = cur * 2;
            if (child > cnt) break;
            if (child+1 <= cnt && cmp(v[child+1], v[child])) child += 1;
            if (!cmp(v[child], v[cur])) break;
            swap(v[cur], v[child]);
            cur = child;
        }
    }

    bool cmp(T& a, T& b) {
        return is_max ? a > b : a < b;
    }
};
