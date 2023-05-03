// 75 lines 47 code 20 comments 8 blanks

// redscript allows line comments
/* as well as block comments */

// it supports global functions
func add2(x: Int32, y: Int32) -> Int32 {
  return x + y;
}

// functions without a type annotation default to Void return type
func tutorial() {
  let x: Int32 = 10;
  // compiler can infer types for local variables, y will be Int32
  let y = 20;
  // it supports arithmetic
  let sum = x + y + 13;
  // as well as mutation
  let mutable = 0;
  mutable += 10;
  // numbers with decimal points default to type Float
  let num = 10.0;
  // you can cast between some types
  let uint: Uint8 = Cast(10);
  // array literals
  let arr = [1, 2, 3];
  // array iteration
  for item in arr {
    // logging and string operations
    Log("at " + ToString(item));
  }
}

// you can define your own classes
public class IntTuple {
  let fst: Int32;
  let snd: Int32;

  // you can define static member functions
  public static func Create(fst: Int32, snd: Int32) -> ref<IntTuple> {
    let tuple = new IntTuple();
    tuple.fst = fst;
    tuple.snd = snd;
    return tuple;
  }
 
  public func Swap() {
    let tmp = this.fst;
    this.fst = this.snd;
    this.snd = tmp;
  }
}

// you can replace existing in-game methods by specifying the class they belong to
@replaceMethod(CraftingSystem)
private final func ProcessCraftSkill(xpAmount: Int32, craftedItem: StatsObjectID) {
  // instantiate a class using the new operator
  let xpEvent = new ExperiencePointsEvent();
  xpEvent.amount = xpAmount * 100;
  xpEvent.type = gamedataProficiencyType.Crafting;
  GetPlayer(this.GetGameInstance()).QueueEvent(xpEvent);
}

// you can add new methods to existing classes as well
// they are visible to other code using the class
@addMethod(BackpackMainGameController)
private final func DisassembleAllJunkItems() -> Void {
  let items = this.m_InventoryManager.GetPlayerItemsByType(gamedataItemType.Gen_Junk);
  let i = 0;
  for item in items {
    ItemActionsHelper.DisassembleItem(this.m_player, InventoryItemData.GetID(item));
  };
  // some methods require CName literals, they need to be prefixed with the n letter
  this.PlaySound(n"Item", n"OnBuy");
}