import org.junit.Test;

public class ChopTest {
    @Test
    public void emptyChopRecursive() {
      Chop chop = new ChopRecursive();
      int index = chop.chop(new Integer[0], 3);
      assertEquals(-1, index);
    }
}
