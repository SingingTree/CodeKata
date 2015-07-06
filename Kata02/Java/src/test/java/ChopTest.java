import org.junit.Test;
import static org.junit.Assert.assertEquals;

public class ChopTest {
    @Test
    public void emptyChopRecursive() {
      Chop<Integer> chop = new ChopRecursive<Integer>();
      int index = chop.chop(new Integer[0], 3);
      assertEquals(-1, index);
    }
}
