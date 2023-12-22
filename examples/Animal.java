package examples;

interface Animal {
    void makeSound();
}

abstract class Mammal {
    abstract void move();
}

class Dog extends Mammal implements Animal {
    public void makeSound() {
        System.out.println("Bark");
    }

    public void move() {
        System.out.println("Run");
    }
}