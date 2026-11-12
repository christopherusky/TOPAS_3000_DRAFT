/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/IncidentType.java
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
 * Classification of the incident.
 *
 * Classification of incident (78 values from the UTMC specification).
 *
 * The XSD carries only a catalogue reference, so the value set is not in it.
 */
public enum IncidentType {
    ABNORMAL_LOAD, AIR_CRASH, AIR_RAID, ANIMALS_ON_ROAD, ATTACK_ON_VEHICLE, AVALANCHE, BLASTING, BRIDGE_BLOCKED, BRIDGE_CLOSED, BRIDGE_DEMOLITION, BRIDGE_MAINTENANCE, BRIDGE_OPENING, BROKEN_DOWN_BUS, BROKEN_DOWN_LORRY, BROKEN_DOWN_VEHICLE, BURST_PIPE, BURST_WATER_MAIN, BUS_DISRUPTION, BUS_LANE_BLOCKED, BUS_LANE_CLOSED, CENTRE_LANE_BLOCKED, CENTRE_LANE_CLOSED, CHEMICAL_SPILLAGE, CHILDREN_ON_ROAD, CIVIL_EMERGENCY, CIVIL_EMERGENCY_CANCELLED, CONGESTION, CONTRAFLOW, CONVOY, CROWD, CYCLISTS, DIVERSION, EMERGENCY_ALERT, FACILITIES_CLOSED, FAULT, FIRE, FLOOD, FLOW, FOG, GAWKING_TRAFFIC, GUNFIRE, HAIL, HEAVY_FLOW, ICE, LANE_CLOSURE, LIGHT_FAULT, OBSTRUCTION, OTHER, PEOPLE_ON_ROAD, POLLUTION, POWER_FAILURE, PUBLIC_DISTURBANCE, QUEUE, RADIOACTIVE_LEAK, RAIN, RIOT, ROADSIDE_FIRE, ROAD_CLOSURE, SANDSTORMS, SECURITY_ALERT, SECURITY_INCIDENT, SIGHTSEERS, SMOG_ALERT, SNOW, SPEED_RESTRICTION, SPILLAGE, SPRAY_HAZARD, STRUCTURAL_DAMAGE, TELEPHONE_FAULT, TEMPORARY_SIGNAL, TERRORIST_INCIDENT, TORNADOES, TRAFFIC_SIGNAL_FAULT, TRAIN_DISRUPTION, UNEXPLAINED_DAMAGE, UNEXPLAINED_EVENT, WASHOUT, WIND;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ABNORMAL_LOAD: return "abnormalLoad";
            case AIR_CRASH: return "airCrash";
            case AIR_RAID: return "airRaid";
            case ANIMALS_ON_ROAD: return "animalsOnRoad";
            case ATTACK_ON_VEHICLE: return "attackOnVehicle";
            case AVALANCHE: return "avalanche";
            case BLASTING: return "blasting";
            case BRIDGE_BLOCKED: return "bridgeBlocked";
            case BRIDGE_CLOSED: return "bridgeClosed";
            case BRIDGE_DEMOLITION: return "bridgeDemolition";
            case BRIDGE_MAINTENANCE: return "bridgeMaintenance";
            case BRIDGE_OPENING: return "bridgeOpening";
            case BROKEN_DOWN_BUS: return "brokenDownBus";
            case BROKEN_DOWN_LORRY: return "brokenDownLorry";
            case BROKEN_DOWN_VEHICLE: return "brokenDownVehicle";
            case BURST_PIPE: return "burstPipe";
            case BURST_WATER_MAIN: return "burstWaterMain";
            case BUS_DISRUPTION: return "busDisruption";
            case BUS_LANE_BLOCKED: return "busLaneBlocked";
            case BUS_LANE_CLOSED: return "busLaneClosed";
            case CENTRE_LANE_BLOCKED: return "centreLaneBlocked";
            case CENTRE_LANE_CLOSED: return "centreLaneClosed";
            case CHEMICAL_SPILLAGE: return "chemicalSpillage";
            case CHILDREN_ON_ROAD: return "childrenOnRoad";
            case CIVIL_EMERGENCY: return "civilEmergency";
            case CIVIL_EMERGENCY_CANCELLED: return "civilEmergencyCancelled";
            case CONGESTION: return "congestion";
            case CONTRAFLOW: return "contraflow";
            case CONVOY: return "convoy";
            case CROWD: return "crowd";
            case CYCLISTS: return "cyclists";
            case DIVERSION: return "diversion";
            case EMERGENCY_ALERT: return "emergencyAlert";
            case FACILITIES_CLOSED: return "facilitiesClosed";
            case FAULT: return "fault";
            case FIRE: return "fire";
            case FLOOD: return "flood";
            case FLOW: return "flow";
            case FOG: return "fog";
            case GAWKING_TRAFFIC: return "gawkingTraffic";
            case GUNFIRE: return "gunfire";
            case HAIL: return "hail";
            case HEAVY_FLOW: return "heavyFlow";
            case ICE: return "ice";
            case LANE_CLOSURE: return "laneClosure";
            case LIGHT_FAULT: return "lightFault";
            case OBSTRUCTION: return "obstruction";
            case OTHER: return "other";
            case PEOPLE_ON_ROAD: return "peopleOnRoad";
            case POLLUTION: return "pollution";
            case POWER_FAILURE: return "powerFailure";
            case PUBLIC_DISTURBANCE: return "publicDisturbance";
            case QUEUE: return "queue";
            case RADIOACTIVE_LEAK: return "radioactiveLeak";
            case RAIN: return "rain";
            case RIOT: return "riot";
            case ROADSIDE_FIRE: return "roadsideFire";
            case ROAD_CLOSURE: return "roadClosure";
            case SANDSTORMS: return "sandstorms";
            case SECURITY_ALERT: return "securityAlert";
            case SECURITY_INCIDENT: return "securityIncident";
            case SIGHTSEERS: return "sightseers";
            case SMOG_ALERT: return "smogAlert";
            case SNOW: return "snow";
            case SPEED_RESTRICTION: return "speedRestriction";
            case SPILLAGE: return "spillage";
            case SPRAY_HAZARD: return "sprayHazard";
            case STRUCTURAL_DAMAGE: return "structuralDamage";
            case TELEPHONE_FAULT: return "telephoneFault";
            case TEMPORARY_SIGNAL: return "temporarySignal";
            case TERRORIST_INCIDENT: return "terroristIncident";
            case TORNADOES: return "tornadoes";
            case TRAFFIC_SIGNAL_FAULT: return "trafficSignalFault";
            case TRAIN_DISRUPTION: return "trainDisruption";
            case UNEXPLAINED_DAMAGE: return "unexplainedDamage";
            case UNEXPLAINED_EVENT: return "unexplainedEvent";
            case WASHOUT: return "washout";
            case WIND: return "wind";
        }
        return null;
    }

    @JsonCreator
    public static IncidentType forValue(String value) throws IOException {
        if (value.equals("abnormalLoad")) return ABNORMAL_LOAD;
        if (value.equals("airCrash")) return AIR_CRASH;
        if (value.equals("airRaid")) return AIR_RAID;
        if (value.equals("animalsOnRoad")) return ANIMALS_ON_ROAD;
        if (value.equals("attackOnVehicle")) return ATTACK_ON_VEHICLE;
        if (value.equals("avalanche")) return AVALANCHE;
        if (value.equals("blasting")) return BLASTING;
        if (value.equals("bridgeBlocked")) return BRIDGE_BLOCKED;
        if (value.equals("bridgeClosed")) return BRIDGE_CLOSED;
        if (value.equals("bridgeDemolition")) return BRIDGE_DEMOLITION;
        if (value.equals("bridgeMaintenance")) return BRIDGE_MAINTENANCE;
        if (value.equals("bridgeOpening")) return BRIDGE_OPENING;
        if (value.equals("brokenDownBus")) return BROKEN_DOWN_BUS;
        if (value.equals("brokenDownLorry")) return BROKEN_DOWN_LORRY;
        if (value.equals("brokenDownVehicle")) return BROKEN_DOWN_VEHICLE;
        if (value.equals("burstPipe")) return BURST_PIPE;
        if (value.equals("burstWaterMain")) return BURST_WATER_MAIN;
        if (value.equals("busDisruption")) return BUS_DISRUPTION;
        if (value.equals("busLaneBlocked")) return BUS_LANE_BLOCKED;
        if (value.equals("busLaneClosed")) return BUS_LANE_CLOSED;
        if (value.equals("centreLaneBlocked")) return CENTRE_LANE_BLOCKED;
        if (value.equals("centreLaneClosed")) return CENTRE_LANE_CLOSED;
        if (value.equals("chemicalSpillage")) return CHEMICAL_SPILLAGE;
        if (value.equals("childrenOnRoad")) return CHILDREN_ON_ROAD;
        if (value.equals("civilEmergency")) return CIVIL_EMERGENCY;
        if (value.equals("civilEmergencyCancelled")) return CIVIL_EMERGENCY_CANCELLED;
        if (value.equals("congestion")) return CONGESTION;
        if (value.equals("contraflow")) return CONTRAFLOW;
        if (value.equals("convoy")) return CONVOY;
        if (value.equals("crowd")) return CROWD;
        if (value.equals("cyclists")) return CYCLISTS;
        if (value.equals("diversion")) return DIVERSION;
        if (value.equals("emergencyAlert")) return EMERGENCY_ALERT;
        if (value.equals("facilitiesClosed")) return FACILITIES_CLOSED;
        if (value.equals("fault")) return FAULT;
        if (value.equals("fire")) return FIRE;
        if (value.equals("flood")) return FLOOD;
        if (value.equals("flow")) return FLOW;
        if (value.equals("fog")) return FOG;
        if (value.equals("gawkingTraffic")) return GAWKING_TRAFFIC;
        if (value.equals("gunfire")) return GUNFIRE;
        if (value.equals("hail")) return HAIL;
        if (value.equals("heavyFlow")) return HEAVY_FLOW;
        if (value.equals("ice")) return ICE;
        if (value.equals("laneClosure")) return LANE_CLOSURE;
        if (value.equals("lightFault")) return LIGHT_FAULT;
        if (value.equals("obstruction")) return OBSTRUCTION;
        if (value.equals("other")) return OTHER;
        if (value.equals("peopleOnRoad")) return PEOPLE_ON_ROAD;
        if (value.equals("pollution")) return POLLUTION;
        if (value.equals("powerFailure")) return POWER_FAILURE;
        if (value.equals("publicDisturbance")) return PUBLIC_DISTURBANCE;
        if (value.equals("queue")) return QUEUE;
        if (value.equals("radioactiveLeak")) return RADIOACTIVE_LEAK;
        if (value.equals("rain")) return RAIN;
        if (value.equals("riot")) return RIOT;
        if (value.equals("roadsideFire")) return ROADSIDE_FIRE;
        if (value.equals("roadClosure")) return ROAD_CLOSURE;
        if (value.equals("sandstorms")) return SANDSTORMS;
        if (value.equals("securityAlert")) return SECURITY_ALERT;
        if (value.equals("securityIncident")) return SECURITY_INCIDENT;
        if (value.equals("sightseers")) return SIGHTSEERS;
        if (value.equals("smogAlert")) return SMOG_ALERT;
        if (value.equals("snow")) return SNOW;
        if (value.equals("speedRestriction")) return SPEED_RESTRICTION;
        if (value.equals("spillage")) return SPILLAGE;
        if (value.equals("sprayHazard")) return SPRAY_HAZARD;
        if (value.equals("structuralDamage")) return STRUCTURAL_DAMAGE;
        if (value.equals("telephoneFault")) return TELEPHONE_FAULT;
        if (value.equals("temporarySignal")) return TEMPORARY_SIGNAL;
        if (value.equals("terroristIncident")) return TERRORIST_INCIDENT;
        if (value.equals("tornadoes")) return TORNADOES;
        if (value.equals("trafficSignalFault")) return TRAFFIC_SIGNAL_FAULT;
        if (value.equals("trainDisruption")) return TRAIN_DISRUPTION;
        if (value.equals("unexplainedDamage")) return UNEXPLAINED_DAMAGE;
        if (value.equals("unexplainedEvent")) return UNEXPLAINED_EVENT;
        if (value.equals("washout")) return WASHOUT;
        if (value.equals("wind")) return WIND;
        throw new IOException("Cannot deserialize IncidentType");
    }
}
