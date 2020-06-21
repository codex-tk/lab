package main

import "fmt"

func add(x, y int) int {
	return x + y
}

func swap(x, y string) (string, string) {
	return y, x
}

func split(sum int) (x, y int) {
	x = sum * 4 / 9
	y = sum - x
	return
}

var x, y, z = 0, 0.1, "z"

const (
	Big   = 1 << 100
	Small = Big >> 99
)

func needInt(x int) int {
	return x*10 + 1
}

func needFloat(x float64) float64 {
	return x * 0.1
}

func main() {
	/*
		fmt.Println("Hello Go!", add(1, 2))
		a, b := swap("a", "b")
		fmt.Println(a, b)
		fmt.Println(split(17))

		q, w, e := 1, 2, 3

		const World = "world"
		fmt.Println(x, y, z, q, w, e, World)
	*/
	/*
		fmt.Println(needInt(Small))
		fmt.Println(needFloat(Small))
		fmt.Println(needFloat(Big))
	*/
	var sum int = 0
	for i := 0; i < 10; i++ {
		sum += i
	}
	fmt.Println(sum)
}
