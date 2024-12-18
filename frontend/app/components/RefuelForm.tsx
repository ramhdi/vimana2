import { useState } from "react";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";

interface RefuelFormProps {
  vehicles: { id: string; brand: string; model: string }[];
  onSubmit: (data: {
    vehicleId: string;
    odometerValue: number;
    refuelQuantity: number;
  }) => void;
}

export function RefuelForm({ vehicles, onSubmit }: RefuelFormProps) {
  const [vehicleId, setVehicleId] = useState("");
  const [odometerValue, setOdometerValue] = useState<number | string>("");
  const [refuelQuantity, setRefuelQuantity] = useState<number | string>("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!vehicleId || !odometerValue || !refuelQuantity) {
      alert("Please fill in all fields.");
      return;
    }
    onSubmit({
      vehicleId,
      odometerValue: Number(odometerValue),
      refuelQuantity: Number(refuelQuantity),
    });
    setVehicleId("");
    setOdometerValue("");
    setRefuelQuantity("");
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4 bg-white p-6 rounded-lg shadow">
      {/* Vehicle Dropdown */}
      <div>
        <label htmlFor="vehicle" className="block text-sm font-medium text-gray-700">
          Select Vehicle
        </label>
        <select
          id="vehicle"
          className="mt-1 block w-full p-2 border rounded"
          value={vehicleId}
          onChange={(e) => setVehicleId(e.target.value)}
          required
        >
          <option value="" disabled>
            Select a vehicle
          </option>
          {vehicles.map((vehicle) => (
            <option key={vehicle.id} value={vehicle.id}>
              {vehicle.brand} {vehicle.model}
            </option>
          ))}
        </select>
      </div>

      {/* Odometer Input */}
      <div>
        <label htmlFor="odometer" className="block text-sm font-medium text-gray-700">
          Odometer Reading (km)
        </label>
        <Input
          id="odometer"
          type="number"
          step="0.01"
          min="0.01"
          value={odometerValue}
          onChange={(e) => setOdometerValue(e.target.value)}
          required
        />
      </div>

      {/* Fuel Quantity Input */}
      <div>
        <label htmlFor="quantity" className="block text-sm font-medium text-gray-700">
          Fuel Quantity (L)
        </label>
        <Input
          id="quantity"
          type="number"
          step="0.01"
          min="0.01"
          value={refuelQuantity}
          onChange={(e) => setRefuelQuantity(e.target.value)}
          required
        />
      </div>

      {/* Submit Button */}
      <Button type="submit" className="w-full">
        Submit
      </Button>
    </form>
  );
}
