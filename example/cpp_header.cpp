#include <cassert>
#include <cctype>

#include <iostream>
#include <string>
#include <vector>
#include <initializer_list>
#include <algorithm>
#include <numeric>

using Int = unsigned;

class u8 : public std::vector<Int> {
public:
    u8() = default;
    u8(const u8 &) = default;
    u8(u8 &&) = default;
    u8(const std::vector<Int> &s) : std::vector<Int>(s) {}
    u8(std::vector<Int> &&s) : std::vector<Int>(s) {}
    u8(std::initializer_list<Int> s) : std::vector<Int>(s) {}
    explicit u8(Int x) : u8({x}) {}
    u8 &operator=(const u8 &) = default;
    u8 &operator=(u8 &&) = default;
    ~u8() = default;

    friend u8 operator|(const u8 &lhs, const u8 &rhs) {
        u8 v = lhs;
        v.insert(v.end(), rhs.begin(), rhs.end());
        return v;
    }
    friend u8 operator+(const u8 &lhs, const u8 &rhs) {
        if (lhs.size() > rhs.size()) return rhs + lhs;
        u8 v = lhs;
        for (std::size_t i = 0; i < lhs.size(); ++i) v.at(i) += rhs.at(i);
        return v;
    }
    friend u8 operator-(const u8 &lhs, const u8 &rhs) {
        if (lhs.size() > rhs.size()) return rhs - lhs;
        u8 v = lhs;
        for (std::size_t i = 0; i < lhs.size(); ++i) v.at(i) -= rhs.at(i);
        return v;
    }
    friend u8 operator*(const u8 &lhs, const u8 &rhs) {
        if (lhs.size() > rhs.size()) return rhs * lhs;
        u8 v = lhs;
        for (std::size_t i = 0; i < lhs.size(); ++i) v.at(i) *= rhs.at(i);
        return v;
    }

    explicit operator bool() const {
        return !empty() && std::all_of(begin(), end(), [](int x) {
            return bool(x);
        });
    }
    friend bool less(const u8 &x, const u8 &y) {
        return std::lexicographical_compare(x.begin(), x.end(), y.begin(), y.end());
    }
    friend bool eq(const u8 &x, const u8 &y) {
        return std::equal(x.begin(), x.end(), y.begin(), y.end());
    }
    friend u8 operator<(const u8 &x, const u8 &y) { return u8(Int(less(x, y))); }
    friend u8 operator>(const u8 &x, const u8 &y) { return u8(Int(less(y, x))); }
    friend u8 operator==(const u8 &x, const u8 &y) { return u8(Int(eq(x, y))); }
    friend u8 operator!=(const u8 &x, const u8 &y) { return u8(Int(!eq(x, y))); }
    friend u8 operator<=(const u8 &x, const u8 &y) { return u8(Int(!less(y, x))); }
    friend u8 operator>=(const u8 &x, const u8 &y) { return u8(Int(!less(x, y))); }

    u8 operator[](const u8 &y) const {
        u8 z;
        for (std::size_t i = 0; i < y.size(); ++i) {
            std::size_t index = y.at(i);
            assert(index < size());
            z.push_back(at(index));
        }
        return z;
    }
    u8 &set(const u8 &y, const u8 &z) {
        assert(!z.empty());
        for (std::size_t i = 0; i < y.size(); ++i) {
            std::size_t index = y.at(i);
            assert(index < size());
            at(index) = z.at(i % z.size());
        }
        return *this;
    }

    friend std::ostream &operator<<(std::ostream &out, const u8 &x) {
        out << "(";
        for (std::size_t i = 0; i < x.size(); ++i) {
            std::cout << x.at(i);
            if (i + 1 < x.size()) std::cout << " | ";
        }
        return out << ")";
    }

    Int sum() const { return std::accumulate(begin(), end(), Int()); }
};

u8 readline() {
    std::string s;
    std::getline(std::cin, s);
    u8 a;
    for (auto x : s) a.push_back(x);
    return a;
}

u8 print(u8 a) {
    std::cout << a << '\n';
    return u8();
}
template <typename ...Args> u8 print(u8 a, Args... args) {
    std::cout << a << ' ';
    return print(args...);
}

u8 sprint(u8 a) {
    for (auto x : a) std::cout << char(x);
    std::cout << '\n';
    return u8();
}
template <typename ...Args> u8 sprint(u8 a, Args... args) {
    for (auto x : a) std::cout << char(x);
    std::cout << ' ';
    return sprint(args...);
}

u8 cyber(u8 a) {
    return u8(std::vector<Int>(a.sum()));
}

u8 trim(u8 a) {
    std::reverse(a.begin(), a.end());
    while (!a.empty() && isspace(a.back())) a.pop_back();
    std::reverse(a.begin(), a.end());
    while (!a.empty() && isspace(a.back())) a.pop_back();
    return a;
}

u8 len(u8 a) {
    return u8({Int(a.size())});
}

u8 slice(u8 a, u8 l, u8 r) {
    u8 b;
    b.assign(a.begin() + l.sum(), a.begin() + r.sum());
    return b;
}

int main() {
    
    return 0;
}
