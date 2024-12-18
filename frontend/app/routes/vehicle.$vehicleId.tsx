import { useState, useEffect } from "react";
import { useParams } from "@remix-run/react";
import { Navbar } from "~/components/Navbar";
import { VehicleDetails } from "~/components/VehicleDetails";
import { AnalysisSection } from "~/components/AnalysisSection";
import { DeleteVehicleModal } from "~/components/Modals/DeleteVehicleModal";
import { vehicleService } from "~/services/vehicleService";
import { Vehicle } from "~/types/vehicle";
import { VehicleModal } from "~/components/Modals/VehicleModal";

export default function VehiclePage() {
  const { vehicleId } = useParams();
  const [vehicle, setVehicle] = useState<Vehicle | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  const [isUpdateModalOpen, setIsUpdateModalOpen] = useState(false);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);

  useEffect(() => {
    const fetchVehicle = async () => {
      if (!vehicleId) {
        setIsError(true);
        return;
      }

      setIsLoading(true);
      try {
        const fetchedVehicle = await vehicleService.fetchVehicleById(vehicleId);
        setVehicle(fetchedVehicle);
        setIsError(false);
      } catch (error) {
        console.error("Error fetching vehicle:", error);
        setIsError(true);
      } finally {
        setIsLoading(false);
      }
    };

    fetchVehicle();
  }, [vehicleId]);

  const handleUpdateVehicle = async (updatedData: Vehicle) => {
    try {
      await vehicleService.updateVehicle(updatedData);
      setVehicle(updatedData); // Update local state
      setIsUpdateModalOpen(false);
    } catch (error) {
      console.error("Error updating vehicle:", error);
      alert("Failed to update vehicle.");
    }
  };

  const handleDeleteVehicle = async () => {
    if (!vehicle) return;

    try {
      await vehicleService.deleteVehicle(vehicle.id);
      window.location.href = "/home"; // Redirect to the home page
    } catch (error) {
      console.error("Error deleting vehicle:", error);
      alert("Failed to delete vehicle.");
    }
  };

  return (
    <>
      <Navbar />
      <div className="container mx-auto p-4 space-y-6">
        <div className="flex flex-col lg:flex-row gap-6">
          {isLoading ? (
            <p className="text-center text-gray-500 mt-6">Loading vehicle details...</p>
          ) : (isError || !vehicle) ? (
            <p className="text-center text-gray-500 mt-6">Failed to load vehicle. Please try again later.</p>
          ) : (
            <>
              {/* Vehicle Details */}
              <div className="lg:w-1/3">
                <VehicleDetails
                  vehicle={vehicle}
                  onEdit={() => setIsUpdateModalOpen(true)}
                  onDelete={() => setIsDeleteModalOpen(true)}
                />
              </div>

              {/* Usage & Performance Analysis */}
              <div className="lg:w-2/3">
                <AnalysisSection vehicleId={vehicleId ?? ""} />
              </div>
            </>
          )}
        </div>
      </div>

      {/* Modals */}
      <VehicleModal
        isOpen={isUpdateModalOpen}
        onClose={() => setIsUpdateModalOpen(false)}
        onSubmit={handleUpdateVehicle}
        vehicle={vehicle}
        mode={"edit"}
      />

      <DeleteVehicleModal
        isOpen={isDeleteModalOpen}
        onClose={() => setIsDeleteModalOpen(false)}
        vehicle={vehicle}
        onConfirm={handleDeleteVehicle}
      />
    </>
  );
}