import { APIResult } from "./api_result";
import { WeeklyExpenses } from "./expense";

export interface Calculations {

    /**
     * Gibt das restliche "Guthaben" für diesen Monat aus.
     * Abzüglich der kommenden und der getätigten Ausgaben.
     * @param month 
     * @param year 
     */
    calculate_leftover(
        month: number, 
        year: number
    ): APIResult<number>

    /**
     * Gibt die mögliche Einsparung an abhängig von der Eingabe des Zeitraums und Modifiers.
     * @param id Ausgabe oder Einnahme
     * @param timeframe_modifier 
     * @param timeframe 
     */
    calculate_savings(
        id: string,
        timeframe_modifier: number,
        timeframe: TimeFrame
    ): APIResult<number>

    /**
     * Gibt die Ausgaben für eine bestimmte Kalenderwoche zurück.
     * z.B. für die Anzeige in einem Graph Mo-So + Total
     * @param week 
     */
    show_expenses(
        week: number
    ): APIResult<WeeklyExpenses>

}

enum TimeFrame {
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY
}