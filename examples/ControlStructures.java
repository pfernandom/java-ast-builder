package examples;

public class ControlStructures {
    public void test() {
        int number = 5;
        if (number > 0) {
            System.out.println("Positive");
        } else {
            System.out.println("Non-Positive");
        }

        for (int i = 0; i < 5; i++) {
            System.out.println(i);
        }

        while (number > 0) {
            number--;
        }
    }
}