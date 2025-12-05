package main

import (
	"bytes"
	"container/list"
	"fmt"
	"log"
	"math"
	"math/big"
	"reflect"
	"time"
	"unicode/utf8"
	"unsafe"

	// "math/rand"
	"os"
	"runtime"

	// package name same as directory
	"learn/pack1"
	"learn/trans"
)

// should not use dot imports (ST1001)go-staticcheck
// import . "learn/pack1"

func main() {
	// test := ReturnStr()
	test := pack1.ReturnStr()
	println(test)

	// hello()
	// countTime()
	// other()
	// myEqual()
	// forString()
	// forLoop()
	// forCharacter()
	// rangeString()
	// a()
	// myBuffer()
	// mySlice()
	// myReslice()
	// copy_append_slice()
	// 	myList()
	// sizeOfInt()
	// myTest()
	// myBig()
	// myStruct()
	// embedd_struct()
	// pointer_value()
	// mem()
	interfacePoly()
	myReflect()
}


func myReflect() {
	var x float64 = 3.4
	v := reflect.ValueOf(x)
	// setting a value:
	// v.SetFloat(3.1415) // Error: will panic: reflect.Value.SetFloat using unaddressable value
	fmt.Println("settability of v:", v.CanSet())
	v = reflect.ValueOf(&x) // Note: take the address of x.
	fmt.Println("type of v:", v.Type())
	fmt.Println("settability of v:", v.CanSet())
	v = v.Elem()
	fmt.Println("The Elem of v is: ", v)
	fmt.Println("settability of v:", v.CanSet())
	v.SetFloat(3.1415) // this works!
	fmt.Println(v.Interface())
	fmt.Println(v)
}

type Shaper interface {
	Area() float32
}

type Square struct {
	side float32
}

func (sq *Square) Area() float32 {
	return sq.side * sq.side
}

type Rectangle struct {
	length, width float32
}

func (r Rectangle) Area() float32 {
	return r.length * r.width
}

func interfacePoly() {

	r := Rectangle{5, 3} // Area() of Rectangle needs a value
	q := &Square{5}      // Area() of Square needs a pointer
	// shapes := []Shaper{Shaper(r), Shaper(q)}
	// or shorter
	shapes := []Shaper{r, q}
	fmt.Println("Looping through shapes for area ...")
	for n, _ := range shapes {
		fmt.Println("Shape details: ", shapes[n])
		fmt.Println("Area of this shape is: ", shapes[n].Area())
	}
}


func mem() {
	var m runtime.MemStats
	runtime.ReadMemStats(&m)
	fmt.Printf("%d Kb\n", m.Alloc/1024)
}

// 1. 外层名字会覆盖内层名字（但是两者的内存空间都保留），这提供了一种重载字段或方法的方式；
// 2. 如果相同的名字在同一级别出现了两次，如果这个名字被程序使用了，将会引发一个错误（不使用没关系）。没有办法来解决这种问题引起的二义性，必须由程序员自己修正。
type A struct {
	ax, ay int
}

type B struct {
	A
	bx, by float32
}

type B2 struct {
	thing int
}

func (b *B2) change() { b.thing = 1 }

func (b B2) write() string { return fmt.Sprint(b) }

func pointer_value() {
	var b1 B2 // b1是值
	b1.change()
	fmt.Println(b1)
	fmt.Println(b1.write())

	b2 := new(B2) // b2是指针
	b2.change()
	fmt.Println(b2.write())
}

func (tn *A) AddThem() int {
	return tn.ax + tn.ay
}

func (tn *B) AddToParam(param float32) float32 {
	return tn.bx + tn.by + param
}

func embedd_struct() {
	b := B{A{1, 2}, 3.0, 4.0}
	fmt.Println(b.ax, b.ay, b.bx, b.by)
	fmt.Println(b.A)
}

// 类型私有
type struct1 struct {
	i1  int
	f1  float32
	str string
}

func NewMatrix(params int) *struct1 {
	m := new(struct1) // 初始化 m
	return m
}

func myStruct() {
	ms := new(struct1)
	ms.i1 = 10
	ms.f1 = 15.5
	ms.str = "Chris"

	fmt.Printf("The int is: %d\n", ms.i1)
	fmt.Printf("The float is: %f\n", ms.f1)
	fmt.Printf("The string is: %s\n", ms.str)
	fmt.Println(ms)

	size := unsafe.Sizeof(ms)
	println(size)
}

func myBig() {
	// Here are some calculations with bigInts:
	im := big.NewInt(math.MaxInt64)
	in := im
	io := big.NewInt(1956)
	ip := big.NewInt(1)
	ip.Mul(im, in).Add(ip, im).Div(ip, io)
	fmt.Printf("Big Int: %v\n", ip)
	// Here are some calculations with bigInts:
	// 创建一个新的大有理数 rm，其值为 math.MaxInt64 / 1956。
	rm := big.NewRat(math.MaxInt64, 1956)
	rn := big.NewRat(-1956, math.MaxInt64)
	ro := big.NewRat(19, 56)
	rp := big.NewRat(1111, 2222)
	rq := big.NewRat(1, 1)
	rq.Mul(rm, rn).Add(rq, ro).Mul(rq, rp)
	fmt.Printf("Big Rat: %v\n", rq)
}

func myTest() {

	var myPrint = log.Print
	where := func() {
		_, file, line, _ := runtime.Caller(1)
		log.Printf("%s:%d", file, line)
	}

	// 闭包调试
	go func() {
		for i := 0; i < 10; i++ {
			myPrint("This is a test log message")
			where()
			time.Sleep(time.Second)
		}
	}()

	// 并发调试
	go func() {
		for i := 0; i < 10; i++ {
			myPrint("This is a test log message")
			where()
			time.Sleep(time.Second)
		}
	}()

	// 并发和 goroutine 泄漏检测
	runtime.SetBlockProfileRate(1)
	time.Sleep(time.Second)

}

// 通过使用 unsafe 包中的方法来测试你电脑上一个整型变量占用多少个字节。
func sizeOfInt() {
	var i int
	fmt.Printf("Size of int: %d bytes\n", unsafe.Sizeof(i))
}

// 使用 container/list 包实现一个双向链表，将 101、102 和 103 放入其中并打印出来。
func myList() {
	list := list.New()
	for _, v := range []int{101, 102, 103} {
		list.PushBack(v)
	}

	for e := list.Front(); e != nil; e = e.Next() {
		// type assertion
		if value, ok := e.Value.(int); ok {
			fmt.Print(value, " ")
		} else {
			fmt.Println("type assertion error")
		}
	}
	fmt.Println()
}

// AppendByte appends byte data to a slice of bytes. If the capacity of the slice is not sufficient,
// it will reallocate a new slice with double the current capacity and copy the original slice to the new one.
// The function then copies the provided data to the new slice and returns the updated slice.
//
// Parameters:
// - slice: The original slice of bytes to which the data will be appended.
// - data: A variadic parameter that accepts one or more bytes to be appended to the slice.
//
// Return:
// - []byte: The updated slice of bytes after appending the provided data.
func AppendByte(slice []byte, data ...byte) []byte {
	m := len(slice)
	n := m + len(data)
	if n > cap(slice) { // if necessary, reallocate
		// allocate double what's needed, for future growth.
		newSlice := make([]byte, (n+1)*2)
		copy(newSlice, slice)
		slice = newSlice
	}
	slice = slice[0:n]
	copy(slice[m:n], data)
	return slice
}

func copy_append_slice() {
	slFrom := []int{1, 2, 3}
	slTo := make([]int, 10)

	n := copy(slTo, slFrom)
	fmt.Println(slTo)
	fmt.Printf("Copied %d elements\n", n) // n == 3

	sl3 := []int{1, 2, 3}
	sl3 = append(sl3, 4, 5, 6)
	fmt.Println(sl3)

}

func myReslice() {
	var ar = [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	var a = ar[5:7]
	for _, v := range a {
		println(v) //5 6
	}
	a = a[0:4]
	for _, v := range a {
		println(v) //5 6 7 8
	}
}

func myBuffer() {
	var buffer bytes.Buffer
	buffer.WriteString("Hello, ")
	buffer.WriteString("World!")
	fmt.Println(buffer.String())
}

func mySlice() {
	// 给定切片 b:= []byte{'g', 'o', 'l', 'a', 'n', 'g'}，那么 b[1:4]、b[:2]、b[2:] 和 b[:] 分别是什么？
	b := []byte{'g', 'o', 'l', 'a', 'n', 'g'}
	fmt.Println(string(b[1:4])) // ola
	fmt.Println(string(b[:2]))  // go
	fmt.Println(string(b[2:]))  // lang
	fmt.Println(string(b[:]))   // golang
	fmt.Println(string(b[:]))
}

func countTime() {
	start := time.Now()
	hello()
	end := time.Now()
	delta := end.Sub(start)
	fmt.Printf("longCalculation took this amount of time: %s\n", delta)
}

func other() {

	var twoPi = 2 * trans.Pi
	fmt.Printf("2*Pi = %g\n", twoPi) // 2*Pi = 6.283185307179586

	var n int16 = 34
	// compiler error: cannot use n (type int16) as type int32 in assignment
	//m = n
	var m int32 = int32(n)
	fmt.Printf("32 bit int is: %d\n", m)
	fmt.Printf("16 bit int is: %d\n", n)

	// 类型别名
	type Rope string
	var r Rope = "Hello, Rope!"
	println(r)

	var ch int = '\u0041'
	var ch2 int = '\u03B2'
	var ch3 int = '\U00101234'
	fmt.Printf("%d - %d - %d\n", ch, ch2, ch3) // integer
	fmt.Printf("%c - %c - %c\n", ch, ch2, ch3) // character
	fmt.Printf("%X - %X - %X\n", ch, ch2, ch3) // UTF-8 bytes
	fmt.Printf("%U - %U - %U", ch, ch2, ch3)   // UTF-8 code point

	println()

	// 非解释字符串
	println(`This is a raw string \n`)
	println("This isn't a raw string \n")

	// 控制结构
	if runtime.GOOS == "windows" {
		println("windows\n")
	} else { // Unix-like
		println("Unix-like\n")
	}

	mySwitch()
	fmt.Print(Season(3))

	// no parentheses
	for i := 0; i < 5; i++ {
		fmt.Printf("This is the %d iteration\n", i)
	}
}

func a() {
	i := 1
	defer fmt.Println(i)
	i += 1
	return
}

func rangeString() {
	str := "Go is a beautiful language!"
	fmt.Printf("The length of str is: %d\n", len(str))
	for pos, char := range str {
		fmt.Printf("Character on position %d is: %c \n", pos, char)
	}
	fmt.Println()
	str2 := "Chinese: 日本語"
	fmt.Printf("The length of str2 is: %d\n", len(str2))
	for pos, char := range str2 {
		fmt.Printf("character %c starts at byte position %d\n", char, pos)
	}
	fmt.Println()
	fmt.Println("index \t int(rune) \t rune \t   char\t bytes")
	for index, rune := range str2 {
		fmt.Printf("%-2d \t     %d    \t  %U  '%c' \t % X\n", index, rune, rune, rune, []byte(string(rune)))
	}
}

func forCharacter() {
	// 1
	for i := 0; i < 25; i++ {
		for j := 0; j <= i; j++ {
			print("G")
		}
		println()
	}

	// 2
	str := "G"
	for i := 0; i < 25; i++ {
		fmt.Println(str)
		str += "G"
	}
}

func forLoop() {
	// 1:
	for i := 0; i < 15; i++ {
		fmt.Printf("This is the %d iteration\n", i)
	}

	// 2:
	i := 0
FLAG:
	if i < 15 {
		fmt.Printf("This is the %d iteration\n", i)
		i += 1
		goto FLAG
	}

}

func getA() int {
	// 局部变量
	a := 50
	return a
}

func Season(month int) string {
	switch month {
	case 12, 1, 2:
		return "Winter"
	case 3, 4, 5:
		return "Spring"
	case 6, 7, 8:
		return "Summer"
	case 9, 10, 11:
		return "Autumn"
	default:
		return "Invalid month"
	}
}

func forString() {
	str := "Go is a beautiful language!"
	fmt.Printf("The length of str is: %d\n", len(str))
	for ix := 0; ix < len(str); ix++ {
		fmt.Printf("Character on position %d is: %c \n", ix, str[ix])
	}
	str2 := "日本語"
	fmt.Printf("The length of str2 is: %d\n", len(str2))
	for ix := 0; ix < len(str2); ix++ {
		fmt.Printf("Character on position %d is: %c \n", ix, str2[ix])
	}
	// ASCII 编码的字符占用 1 个字节，既每个索引都指向不同的字符，而非 ASCII 编码的字符（占有 2 到 4 个字节）不能单纯地使用索引来判断是否为同一个字符。
	fmt.Printf("The length of str2 is: %d\n", utf8.RuneCountInString(str2))
	for ix, v := range str2 {
		fmt.Printf("Character on position %d is: %c \n", ix, v)
	}
}

func hello() {
	// 匿名函数
	h := func() { println("Hello world!") }
	h()
	var goos string = runtime.GOOS
	fmt.Printf("The operating system is: %s\n", goos)
	path := os.Getenv("PATH")
	fmt.Printf("Path is %s\n", path)
	println(getA())
	// 空白标识符
	_ = getA()
}

func myEqual() {
	var aVar = 10
	var bVar = 20
	println(aVar == bVar)
	println(aVar != bVar)
}
func mySwitch() {
	var num1 int = 100
	switch num1 {
	case 98, 99:
		fmt.Println("It's equal to 98")
	case 100:
		fmt.Println("It's equal to 100")
	default:
		fmt.Println("It's not equal to 98 or 100")
	}

	k := 6
	switch k {
	case 4:
		fmt.Println("was <= 4")
		fallthrough
	case 5:
		fmt.Println("was <= 5")
		fallthrough
	case 6:
		fmt.Println("was <= 6")
		fallthrough
	case 7:
		fmt.Println("was <= 7")
		fallthrough
	case 8:
		fmt.Println("was <= 8")
		fallthrough
	default:
		fmt.Println("default case")
	}
	// output
	// was <= 6
	// was <= 7
	// was <= 8
	// default case
}
