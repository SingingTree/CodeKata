import org.junit.Test;
import static org.junit.Assert.assertEquals;

public class ChopTest {
    @Test
    public void noElementsChopRecursive() {
      Chop<Integer> chop = new ChopRecursive<Integer>();
      int index = chop.chop(3, new Integer[0]);
      assertEquals(-1, index);
    }

    @Test
    public void singleElementChopRecursive() {
        Chop<Integer> chop = new ChopRecursive<Integer>();
        Integer[] array = {1};
        int index = chop.chop(3, array);
        assertEquals(-1, index);
        index = chop.chop(1, array);
        assertEquals(0, index);
    }
}
