/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/PlannedEventType.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_event;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Classification of the event.
 *
 * Classification of planned event (32 values from the UTMC specification).
 *
 * The XSD carries only a catalogue reference, so the value set is not in it.
 */
public enum PlannedEventType {
    ATHLETICS, BALL_GAME, BOXING, CYCLE_RACE, DEMONSTRATION, EXHIBITION, FAIR, FESTIVAL, FOOTBALL, FUNFAIR, GOLF, INTERNATIONAL_SPORTS, MARATHON, MARCH, MARKET, MATCH, OTHER, PARADE, PROCESSION, RACE, RUGBY, SHOW, SHOW_JUMPING, SPORTS_EVENT, SPORTS_MEETING, STATE_OCCASION, STRIKE, TENNIS, TOURNAMENT, TRADE_FAIR, WATER_SPORTS, WINTER_SPORTS;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ATHLETICS: return "athletics";
            case BALL_GAME: return "ballGame";
            case BOXING: return "boxing";
            case CYCLE_RACE: return "cycleRace";
            case DEMONSTRATION: return "demonstration";
            case EXHIBITION: return "exhibition";
            case FAIR: return "fair";
            case FESTIVAL: return "festival";
            case FOOTBALL: return "football";
            case FUNFAIR: return "funfair";
            case GOLF: return "golf";
            case INTERNATIONAL_SPORTS: return "internationalSports";
            case MARATHON: return "marathon";
            case MARCH: return "march";
            case MARKET: return "market";
            case MATCH: return "match";
            case OTHER: return "other";
            case PARADE: return "parade";
            case PROCESSION: return "procession";
            case RACE: return "race";
            case RUGBY: return "rugby";
            case SHOW: return "show";
            case SHOW_JUMPING: return "showJumping";
            case SPORTS_EVENT: return "sportsEvent";
            case SPORTS_MEETING: return "sportsMeeting";
            case STATE_OCCASION: return "stateOccasion";
            case STRIKE: return "strike";
            case TENNIS: return "tennis";
            case TOURNAMENT: return "tournament";
            case TRADE_FAIR: return "tradeFair";
            case WATER_SPORTS: return "waterSports";
            case WINTER_SPORTS: return "winterSports";
        }
        return null;
    }

    @JsonCreator
    public static PlannedEventType forValue(String value) throws IOException {
        if (value.equals("athletics")) return ATHLETICS;
        if (value.equals("ballGame")) return BALL_GAME;
        if (value.equals("boxing")) return BOXING;
        if (value.equals("cycleRace")) return CYCLE_RACE;
        if (value.equals("demonstration")) return DEMONSTRATION;
        if (value.equals("exhibition")) return EXHIBITION;
        if (value.equals("fair")) return FAIR;
        if (value.equals("festival")) return FESTIVAL;
        if (value.equals("football")) return FOOTBALL;
        if (value.equals("funfair")) return FUNFAIR;
        if (value.equals("golf")) return GOLF;
        if (value.equals("internationalSports")) return INTERNATIONAL_SPORTS;
        if (value.equals("marathon")) return MARATHON;
        if (value.equals("march")) return MARCH;
        if (value.equals("market")) return MARKET;
        if (value.equals("match")) return MATCH;
        if (value.equals("other")) return OTHER;
        if (value.equals("parade")) return PARADE;
        if (value.equals("procession")) return PROCESSION;
        if (value.equals("race")) return RACE;
        if (value.equals("rugby")) return RUGBY;
        if (value.equals("show")) return SHOW;
        if (value.equals("showJumping")) return SHOW_JUMPING;
        if (value.equals("sportsEvent")) return SPORTS_EVENT;
        if (value.equals("sportsMeeting")) return SPORTS_MEETING;
        if (value.equals("stateOccasion")) return STATE_OCCASION;
        if (value.equals("strike")) return STRIKE;
        if (value.equals("tennis")) return TENNIS;
        if (value.equals("tournament")) return TOURNAMENT;
        if (value.equals("tradeFair")) return TRADE_FAIR;
        if (value.equals("waterSports")) return WATER_SPORTS;
        if (value.equals("winterSports")) return WINTER_SPORTS;
        throw new IOException("Cannot deserialize PlannedEventType");
    }
}
