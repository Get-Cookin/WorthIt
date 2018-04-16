use incomePerDayMaths;
use eventIncomeMaths;
use costOfIngredientsPerDayMaths;
use costOfIngredientsPerEventMaths;
use pitchFeePerDayMaths;
use labourCostsPerEventMaths;
use totalCostPerDayMaths;
use totalCostPerEventMaths;
use eventProfitPerDayMaths;
use totalEventProfitMaths;
use devilsPantryFranchiseFeeMaths;
use netProfitMaths;


#[test]
fn incomePerDayMathsTest() {
    let servingsPerDay = 200.0;
    let incomePerDay = incomePerDayMaths(servingsPerDay);
    assert_eq!(incomePerDay, 1200.0);
}

#[test]
fn eventIncomeMathsTest() {
    let daysOfEvent = 3.0;
    let incomePerDay = 1200.0;
    let eventIncome = eventIncomeMaths(daysOfEvent, incomePerDay);
    assert_eq!(eventIncome, 3600.0);
}

#[test]
fn costOfIngredientsPerDayMathsTest() {
    let servingsPerDay = 200.0;
    let costOfIngredientsPerDay = costOfIngredientsPerDayMaths(servingsPerDay);
    assert_eq!(costOfIngredientsPerDay, 300.0);
}

#[test]
fn costOfIngredientsPerEventMathsTest() {
    let daysOfEvent = 3.0;
    let servingsPerDay = 200.0;
    let costPerDay = costOfIngredientsPerDayMaths(servingsPerDay);
    let costOfIngredientsPerEvent = costOfIngredientsPerEventMaths(costPerDay, daysOfEvent);
    assert_eq!(costOfIngredientsPerEvent, 900.0);
}

#[test]
fn pitchFeePerDayMathsTest() {
    let pitchFeeTotal = 350.0;
    let daysOfEvent = 3.0;
    let pitchFeePerDay = pitchFeePerDayMaths(pitchFeeTotal, daysOfEvent);
    assert_eq!(pitchFeePerDay, 116.66666666666667);
}

#[test]
fn labourCostsPerEventMathsTest() {
    let labourCostsPerDay = 150.0;
    let daysOfEvent = 3.0;
    let labourCostsPerEvent = labourCostsPerEventMaths(labourCostsPerDay, daysOfEvent);
    assert_eq!(labourCostsPerEvent, 450.0);
}

#[test]
fn totalCostPerDayMathsTest() {
    let servingsPerDay = 200.0;
    let daysOfEvent = 3.0;
    let pitchFeeTotal = 350.0;
    let labourCostsPerDay = 150.0;
    let vanHirePerDay = 26.0;
    let totalFuel = 20.0;
    let costOfIngredientsPerDay = costOfIngredientsPerDayMaths(servingsPerDay);
    let pitchFeePerDay = pitchFeePerDayMaths(pitchFeeTotal, daysOfEvent);
    let totalCostPerDay = totalCostPerDayMaths(costOfIngredientsPerDay, pitchFeePerDay, vanHirePerDay, totalFuel, labourCostsPerDay);
    assert_eq!(totalCostPerDay, 612.6666666666667);
}

#[test]
fn totalCostPerEventMathsTest() {
    let servingsPerDay = 200.0;
    let daysOfEvent = 3.0;
    let pitchFeeTotal = 350.0;
    let labourCostsPerDay = 150.0;
    let vanHirePerDay = 26.0;
    let totalFuel = 20.0;
    let costOfIngredientsPerDay = costOfIngredientsPerDayMaths(servingsPerDay);
    let pitchFeePerDay = pitchFeePerDayMaths(pitchFeeTotal, daysOfEvent);
    let totalCostPerDay = totalCostPerDayMaths(costOfIngredientsPerDay, pitchFeePerDay, vanHirePerDay, totalFuel, labourCostsPerDay);
    let totalCostPerEvent = totalCostPerEventMaths(totalCostPerDay, daysOfEvent);
    assert_eq!(totalCostPerEvent, 1838.0000000000002);
}

#[test]
fn eventProfitPerDayMathsTest() {
    let servingsPerDay = 200.0;
    let daysOfEvent = 3.0;
    let pitchFeeTotal = 350.0;
    let labourCostsPerDay = 150.0;
    let vanHirePerDay = 26.0;
    let totalFuel = 20.0;
    let incomePerDay = 1200.0;
    let costOfIngredientsPerDay = costOfIngredientsPerDayMaths(servingsPerDay);
    let pitchFeePerDay = pitchFeePerDayMaths(pitchFeeTotal, daysOfEvent);
    let totalCostPerDay = totalCostPerDayMaths(costOfIngredientsPerDay, pitchFeePerDay, vanHirePerDay, totalFuel, labourCostsPerDay);
    let eventProfitPerDay = eventProfitPerDayMaths(incomePerDay, totalCostPerDay);
    assert_eq!(eventProfitPerDay, 587.3333333333333);
}

#[test]
fn totalEventProfitMathsTest() {
    let servingsPerDay = 200.0;
    let daysOfEvent = 3.0;
    let pitchFeeTotal = 350.0;
    let labourCostsPerDay = 150.0;
    let vanHirePerDay = 26.0;
    let totalFuel = 20.0;
    let incomePerDay = 1200.0;
    let costOfIngredientsPerDay = costOfIngredientsPerDayMaths(servingsPerDay);
    let pitchFeePerDay = pitchFeePerDayMaths(pitchFeeTotal, daysOfEvent);
    let totalCostPerDay = totalCostPerDayMaths(costOfIngredientsPerDay, pitchFeePerDay, vanHirePerDay, totalFuel, labourCostsPerDay);
    let eventProfitPerDay = eventProfitPerDayMaths(incomePerDay, totalCostPerDay);
    let totalEventProfit = totalEventProfitMaths(eventProfitPerDay, daysOfEvent);
    assert_eq!(totalEventProfit, 1761.9999999999998);
}

#[test]
fn devilsPantryFranchiseFeeMathsTest() {
    let servingsPerDay = 200.0;
    let daysOfEvent = 3.0;
    let pitchFeeTotal = 350.0;
    let labourCostsPerDay = 150.0;
    let vanHirePerDay = 26.0;
    let totalFuel = 20.0;
    let incomePerDay = 1200.0;
    let costOfIngredientsPerDay = costOfIngredientsPerDayMaths(servingsPerDay);
    let pitchFeePerDay = pitchFeePerDayMaths(pitchFeeTotal, daysOfEvent);
    let totalCostPerDay = totalCostPerDayMaths(costOfIngredientsPerDay, pitchFeePerDay, vanHirePerDay, totalFuel, labourCostsPerDay);
    let eventProfitPerDay = eventProfitPerDayMaths(incomePerDay, totalCostPerDay);
    let totalEventProfit = totalEventProfitMaths(eventProfitPerDay, daysOfEvent);
    let devilsPantryFranchiseFee = devilsPantryFranchiseFeeMaths(totalEventProfit);
    assert_eq!(devilsPantryFranchiseFee, 352.4);
}

#[test]
fn netProfitMathsTest() {
    let servingsPerDay = 200.0;
    let daysOfEvent = 3.0;
    let pitchFeeTotal = 350.0;
    let labourCostsPerDay = 150.0;
    let vanHirePerDay = 26.0;
    let totalFuel = 20.0;
    let incomePerDay = 1200.0;
    let costOfIngredientsPerDay = costOfIngredientsPerDayMaths(servingsPerDay);
    let pitchFeePerDay = pitchFeePerDayMaths(pitchFeeTotal, daysOfEvent);
    let totalCostPerDay = totalCostPerDayMaths(costOfIngredientsPerDay, pitchFeePerDay, vanHirePerDay, totalFuel, labourCostsPerDay);
    let eventProfitPerDay = eventProfitPerDayMaths(incomePerDay, totalCostPerDay);
    let totalEventProfit = totalEventProfitMaths(eventProfitPerDay, daysOfEvent);
    let devilsPantryFranchiseFee = devilsPantryFranchiseFeeMaths(totalEventProfit);
    let netProfit = netProfitMaths(totalEventProfit, devilsPantryFranchiseFee);
    assert_eq!(netProfit, 1409.6);
}