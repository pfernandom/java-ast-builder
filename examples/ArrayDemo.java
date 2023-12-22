package examples;

public class ArrayDemo {
    public void process() {
        int[] numbers = { 1, 2, 3, 4, 5 };
        try {
            System.out.println(numbers[5]);
        } catch (ArrayIndexOutOfBoundsException e) {
            e.printStackTrace();
        }
    }
}