import { useState, useEffect } from "react";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import { vehicleService } from "~/services/vehicleService";

export function AnalysisSection({ vehicleId }: { vehicleId: string }) {
  const [dateRange, setDateRange] = useState("7d");
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");
  const [traveledDistance, setTraveledDistance] = useState<string | number>("N/A");
  const [fuelEconomy, setFuelEconomy] = useState<string | number>("N/A");
  const [refuelHistory, setRefuelHistory] = useState<any[]>([]);
  const [maintenanceHistory, setMaintenanceHistory] = useState<any[]>([]);

  // Compute start and end dates based on range
  useEffect(() => {
    computeAndSetDates(dateRange);
  }, [dateRange]);

  const computeAndSetDates = (range: string) => {
    const today = new Date();

    // Preserve current startDate and endDate for custom range
    if (range === "custom") {
      return;
    }

    let start: Date | null = null;
    const end = today;

    switch (range) {
      case "1d":
        start = new Date(today); // Today
        break;
      case "7d":
        start = new Date(today.getTime() - 6 * 24 * 60 * 60 * 1000); // Last 7 days
        break;
      case "30d":
        start = new Date(today.getTime() - 29 * 24 * 60 * 60 * 1000); // Last 30 days
        break;
      case "ytd":
        start = new Date(Date.UTC(today.getFullYear(), 0, 1)); // Start of the year in UTC
        break;
      default:
        start = null;
    }

    setStartDate(start ? formatDate(start) : "");
    setEndDate(formatDate(end));
  };

  const formatDate = (date: Date): string => {
    return date.toISOString().split("T")[0]; // Format as YYYY-MM-DD
  };

  const fetchAnalysis = async () => {
    try {
      const analysis = await vehicleService.fetchVehicleAnalysis(vehicleId, startDate, endDate);
      setTraveledDistance(analysis.traveled_distance || "N/A");
      setFuelEconomy(analysis.fuel_economy || "N/A");

      const refuel = await vehicleService.fetchRefuelHistory(vehicleId, startDate, endDate);
      setRefuelHistory(refuel || []);

      const maintenance = await vehicleService.fetchMaintenanceHistory(vehicleId, startDate, endDate);
      setMaintenanceHistory(maintenance || []);
    } catch (error) {
      console.error("Error fetching analysis:", error);
      alert("Failed to fetch analysis data.");
    }
  };

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h5 className="text-lg font-semibold text-gray-800 mb-4">Usage & Performance Analysis</h5>

      {/* Date Range Selector */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <select
          className="col-span-1 bg-gray-50 border rounded p-2"
          value={dateRange}
          onChange={(e) => setDateRange(e.target.value)}
        >
          <option value="1d">Last 1 Day</option>
          <option value="7d">Last 7 Days</option>
          <option value="30d">Last 30 Days</option>
          <option value="ytd">Year to Date</option>
          <option value="custom">Custom</option>
        </select>

        {/* Date Inputs */}
        <Input
          type="date"
          placeholder="Start Date"
          value={startDate}
          onChange={(e) => setStartDate(e.target.value)}
          disabled={dateRange !== "custom"}
        />
        <Input
          type="date"
          placeholder="End Date"
          value={endDate}
          onChange={(e) => setEndDate(e.target.value)}
          disabled={dateRange !== "custom"}
        />
        <Button onClick={fetchAnalysis}>Fetch</Button>
      </div>

      {/* Performance Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div className="p-4 bg-gray-50 rounded shadow">
          <h6 className="font-medium text-gray-600">Traveled Distance</h6>
          <p className="text-lg font-semibold text-gray-800">{traveledDistance}</p>
        </div>
        <div className="p-4 bg-gray-50 rounded shadow">
          <h6 className="font-medium text-gray-600">Average Fuel Economy</h6>
          <p className="text-lg font-semibold text-gray-800">{fuelEconomy}</p>
        </div>
      </div>

      {/* Refuel History */}
      <div className="mt-6">
        <h6 className="font-medium text-gray-600 mb-2">Refuel History</h6>
        <table className="w-full border text-sm">
          <thead className="bg-gray-100">
            <tr>
              <th className="py-2 px-4">Date</th>
              <th className="py-2 px-4">Amount (L)</th>
              <th className="py-2 px-4">Cost</th>
            </tr>
          </thead>
          <tbody>
            {refuelHistory.length > 0 ? (
              refuelHistory.map((entry, index) => (
                <tr key={index}>
                  <td className="py-2 px-4">{entry.date}</td>
                  <td className="py-2 px-4">{entry.amount}</td>
                  <td className="py-2 px-4">{entry.cost}</td>
                </tr>
              ))
            ) : (
              <tr>
                <td className="py-2 px-4 text-center" colSpan={3}>
                  No data available
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      {/* Maintenance History */}
      <div className="mt-6">
        <h6 className="font-medium text-gray-600 mb-2">Maintenance History</h6>
        <table className="w-full border text-sm">
          <thead className="bg-gray-100">
            <tr>
              <th className="py-2 px-4">Date</th>
              <th className="py-2 px-4">Description</th>
              <th className="py-2 px-4">Cost</th>
            </tr>
          </thead>
          <tbody>
            {maintenanceHistory.length > 0 ? (
              maintenanceHistory.map((entry, index) => (
                <tr key={index}>
                  <td className="py-2 px-4">{entry.date}</td>
                  <td className="py-2 px-4">{entry.description}</td>
                  <td className="py-2 px-4">{entry.cost}</td>
                </tr>
              ))
            ) : (
              <tr>
                <td className="py-2 px-4 text-center" colSpan={3}>
                  No data available
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}