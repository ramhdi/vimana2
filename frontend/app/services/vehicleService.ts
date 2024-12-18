import { BASE_URL } from "~/config";
import { Vehicle } from "~/types/vehicle";

const VEHICLE_BASE_URL = `${BASE_URL}/api/protected/vehicles`;

export const vehicleService = {
  async fetchVehicles(): Promise<Vehicle[]> {
    const response = await fetch(`${VEHICLE_BASE_URL}/`, { credentials: "include" });
    if (!response.ok) {
      throw new Error("Failed to fetch vehicles");
    }
    return response.json();
  },

  async fetchVehicleById(vehicleId: string): Promise<Vehicle> {
    const response = await fetch(`${VEHICLE_BASE_URL}/${vehicleId}`, {
      credentials: "include", // Include cookies for auth
    });
    if (!response.ok) {
      throw new Error("Failed to fetch vehicle");
    }
    return response.json();
  },

  async createVehicle(vehicle: Omit<Vehicle, "id">): Promise<Vehicle> {
    const response = await fetch(`${VEHICLE_BASE_URL}/`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(vehicle),
      credentials: "include",
    });
    if (!response.ok) {
      throw new Error("Failed to create vehicle");
    }
    return response.json();
  },

  async updateVehicle(vehicle: Vehicle): Promise<void> {
    const response = await fetch(`${VEHICLE_BASE_URL}/${vehicle.id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(vehicle),
      credentials: "include",
    });
    if (!response.ok) {
      throw new Error("Failed to update vehicle");
    }
  },

  async deleteVehicle(vehicleId: string): Promise<void> {
    const response = await fetch(`${VEHICLE_BASE_URL}/${vehicleId}`, {
      method: "DELETE",
      credentials: "include",
    });
    if (!response.ok) {
      throw new Error("Failed to delete vehicle");
    }
  },

  fetchVehicleAnalysis: async (vehicleId: string, startDate: string, endDate: string) => {
    // const response = await fetch(`/api/protected/odometer/${vehicleId}/traveled?start_date=${startDate}&end_date=${endDate}`, {
    //   credentials: "include",
    // });
    // if (!response.ok) {
    //   throw new Error("Failed to fetch analysis data");
    // }
    // return response.json();
    return {
      traveled_distance: 999,
      fuel_economy: 666,
    };
  },

  fetchRefuelHistory: async (vehicleId: string, startDate: string, endDate: string) => {
    // const response = await fetch(`/api/protected/refuel/${vehicleId}?start_date=${startDate}&end_date=${endDate}`, {
    //   credentials: "include",
    // });
    // if (!response.ok) {
    //   throw new Error("Failed to fetch refuel history");
    // }
    // return response.json();
    return [];
  },

  fetchMaintenanceHistory: async (vehicleId: string, startDate: string, endDate: string) => {
    // const response = await fetch(`/api/protected/maintenance/${vehicleId}?start_date=${startDate}&end_date=${endDate}`, {
    //   credentials: "include",
    // });
    // if (!response.ok) {
    //   throw new Error("Failed to fetch maintenance history");
    // }
    // return response.json();
    return [];
  },
};
