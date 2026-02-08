import java.time.Duration;

public class main {
  public static void main(String[] args) {

    Thread t1 =
        new Thread(
            new Runnable() {
              @Override
              public void run() {
                System.out.println("Thread 1");
              }
            });

    Thread t2 =
        new Thread(
            () -> {
              try {
                Thread.sleep(1145);
                System.out.println("wtf sleep");
                Thread.sleep(Duration.ofMillis(1145).toMillis());;
              } catch (InterruptedException e) {
                  Thread.currentThread().interrupt();
              }
              System.out.println("Thread 2");
            });

    t2.start();
    t1.start();
  }
}
