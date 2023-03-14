# HeRang

## 简介

HeRang，由 `Rust` 编写的下一代高性能脚本语言，适用于**Cyber 空间**。

灵感来源：[helang](https://github.com/kifuan/helang)。

你可以直接运行 `herang` 与**Saint He**面对面对话。

也可以运行 `herang --path path_to_her` 运行 `her` 文件。

运行 `herang --path path_to_her --cpp` 将 HeRang 代码翻译为 C++。

## 语法

`herang` 的标识符只能为大小写字母组成的字符串。

**Saint He**曾说，一切类型均为 `u8`。

```code
a = 1 | 2;
b = 1 | 1 | 4 | 5 | 1 | 4;
print(a, b);

a = 3 | (b = 1 | 2) | 4 | b;
print(a);
```

**Saint He**曾用 108 天开了个灯，`herang` 也支持循环开灯。

```code
a = 0 | 1 | 2 | 3 | 4;
a[a] = 1 | 2;
print(a);
```

`herang` 的表达式从左往右计算。

**Saint He**身处**Cyber 空间**，因此我们可以声明一个 `cyber u8`，内部全为 0。

```code
a = cyber(5);
print(a);
```

**Saint He**曾用 ¥ 进行招聘，我们也可以用 `$` 来招聘函数。

`$` 声明结尾有分号，返回值为最后一个语句的值。

```code
$cyberfive() { cyber(5); };
print(cyberfive());
```

`$` 会修改外部变量，但若参数和外部变量重名，则只会修改参数。

```code
he = 1 | 2 | 6 | 7;
rang = 52 | 57 | 58 | 65;
$keyboard(rang) { he = he | 11; rang = 1 | 1 | 4; };
print(he, rang);

keyboard(rang);
print(he, rang);
```

由于某些限制，直接与**Saint He**对话时只能招聘单行的 `$`。

结合起来，就可以得到一份 **Cyber Code**：

```code
forceCon = cyber(68);
$powerCon(whichKey, Force) { forceCon[whichKey] = Force; };

powerCon(1 | 2 | 6 | 7 | 11 | 52 | 57 | 58 | 65, 10);

print(forceCon);
```

## Hello, World

```code
sprint(72 | 101 | 108 | 108 | 111 | 44, 119 | 111 | 114 | 108 | 100 | 33);
```
