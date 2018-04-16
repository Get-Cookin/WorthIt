#![allow(non_snake_case, non_camel_case_types)]

#[cfg(test)]
mod testsLib;


#[no_mangle]
pub fn incomePerDayMaths(servingsPerDay: f64) -> f64 {
    servingsPerDay * 6.0
}

#[no_mangle]
pub fn eventIncomeMaths(daysOfEvent: f64, incomePerDay: f64) -> f64 {
        daysOfEvent * incomePerDay
}

#[no_mangle]
pub fn costOfIngredientsPerDayMaths(servingsPerDay: f64) -> f64 {
    servingsPerDay * 1.5
}

#[no_mangle]
pub fn costOfIngredientsPerEventMaths(costPerDay: f64, daysOfEvent: f64) -> f64 {
    costPerDay * daysOfEvent
}

#[no_mangle]
pub fn pitchFeePerDayMaths(pitchFeeTotal: f64, daysOfEvent: f64) -> f64 {
    pitchFeeTotal / daysOfEvent
}

#[no_mangle]
pub fn labourCostsPerEventMaths(labourCostsPerDay: f64, daysOfEvent: f64) -> f64 {
    labourCostsPerDay * daysOfEvent
}

#[no_mangle]
pub fn totalCostPerDayMaths(costOfIngredientsPerDay: f64, pitchFeePerDay: f64, vanHirePerDay: f64, totalFuel: f64, labourCostsPerDay: f64) -> f64 {
    costOfIngredientsPerDay + pitchFeePerDay + vanHirePerDay + totalFuel + labourCostsPerDay
}

#[no_mangle]
pub fn totalCostPerEventMaths(totalCostPerDay: f64, daysOfEvent: f64) -> f64 {
    totalCostPerDay * daysOfEvent
}

#[no_mangle]
pub fn eventProfitPerDayMaths(incomePerDay: f64, totalCostPerDay: f64) -> f64 {
    incomePerDay - totalCostPerDay
}

#[no_mangle]
pub fn totalEventProfitMaths(eventProfitPerDay: f64, daysOfEvent: f64) -> f64 {
    eventProfitPerDay * daysOfEvent
}

#[no_mangle]
pub fn devilsPantryFranchiseFeeMaths(totalEventProfit: f64) -> f64 {
    totalEventProfit * 0.2
}

#[no_mangle]
pub fn netProfitMaths(totalEventProfit: f64, devilsPantryFranchiseFee: f64) -> f64 {
    totalEventProfit - devilsPantryFranchiseFee
}