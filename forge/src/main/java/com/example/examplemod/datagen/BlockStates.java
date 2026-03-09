package com.example.examplemod.datagen;

import java.util.function.Function;

import com.example.examplemod.MyMod;
import com.example.examplemod.block.ModBlocks;
import com.example.examplemod.block.custom.CornCropBlock;
import com.example.examplemod.block.custom.StrawberryCropBlock;

import net.minecraft.data.PackOutput;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.ButtonBlock;
import net.minecraft.world.level.block.CropBlock;
import net.minecraft.world.level.block.DoorBlock;
import net.minecraft.world.level.block.FenceBlock;
import net.minecraft.world.level.block.FenceGateBlock;
import net.minecraft.world.level.block.PressurePlateBlock;
import net.minecraft.world.level.block.SlabBlock;
import net.minecraft.world.level.block.StairBlock;
import net.minecraft.world.level.block.TrapDoorBlock;
import net.minecraft.world.level.block.WallBlock;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraftforge.client.model.generators.BlockStateProvider;
import net.minecraftforge.client.model.generators.ConfiguredModel;
import net.minecraftforge.client.model.generators.ModelFile;
import net.minecraftforge.common.data.ExistingFileHelper;
import net.minecraftforge.registries.RegistryObject;

public class BlockStates extends BlockStateProvider {

  public BlockStates(PackOutput output, ExistingFileHelper exFileHelper) {
    super(output, MyMod.MODID, exFileHelper);
  }

  @Override
  protected void registerStatesAndModels() {

    simpleBlockWithItem(
        ModBlocks.GEM_POLISHING_STATION.get(),
        new ModelFile.UncheckedModelFile(modLoc("block/gem_polishing_station")));

    simpleBlockWithItem(
        ModBlocks.CATMINT.get(),
        models()
            .cross(
                blockTexture(ModBlocks.CATMINT.get()).getPath(),
                blockTexture(ModBlocks.CATMINT.get()))
            .renderType("cutout"));
    simpleBlockWithItem(
        ModBlocks.POTTED_CATMINT.get(),
        models()
            .singleTexture(
                "potted_catmint",
                ResourceLocation.parse("flower_pot_cross"),
                "plant",
                blockTexture(ModBlocks.CATMINT.get()))
            .renderType("cutout"));

    blockWithItem(ModBlocks.SAPPHIRE_BLOCK);
    blockWithItem(ModBlocks.RAW_SAPPHIRE_BLOCK);

    blockWithItem(ModBlocks.SAPPHIRE_ORE);
    blockWithItem(ModBlocks.DEEPSLATE_SAPPHIRE_ORE);
    blockWithItem(ModBlocks.NETHER_SAPPHIRE_ORE);
    blockWithItem(ModBlocks.END_STONE_SAPPHIRE_ORE);

    blockWithItem(ModBlocks.SOUND_BLOCK);
    blockWithItem(ModBlocks.WTF);

    stairsBlock(
        ((StairBlock) ModBlocks.SAPPHIRE_STAIRS.get()),
        blockTexture(ModBlocks.SAPPHIRE_BLOCK.get()));
    slabBlock(
        ((SlabBlock) ModBlocks.SAPPHIRE_SLAB.get()),
        blockTexture(ModBlocks.SAPPHIRE_BLOCK.get()),
        blockTexture(ModBlocks.SAPPHIRE_BLOCK.get()));

    buttonBlock(
        ((ButtonBlock) ModBlocks.SAPPHIRE_BUTTON.get()),
        blockTexture(ModBlocks.SAPPHIRE_BLOCK.get()));
    pressurePlateBlock(
        ((PressurePlateBlock) ModBlocks.SAPPHIRE_PRESSURE_PLATE.get()),
        blockTexture(ModBlocks.SAPPHIRE_BLOCK.get()));

    fenceBlock(
        ((FenceBlock) ModBlocks.SAPPHIRE_FENCE.get()),
        blockTexture(ModBlocks.SAPPHIRE_BLOCK.get()));
    fenceGateBlock(
        ((FenceGateBlock) ModBlocks.SAPPHIRE_FENCE_GATE.get()),
        blockTexture(ModBlocks.SAPPHIRE_BLOCK.get()));
    wallBlock(
        ((WallBlock) ModBlocks.SAPPHIRE_WALL.get()), blockTexture(ModBlocks.SAPPHIRE_BLOCK.get()));

    doorBlockWithRenderType(
        ((DoorBlock) ModBlocks.SAPPHIRE_DOOR.get()),
        modLoc("block/sapphire_door_bottom"),
        modLoc("block/sapphire_door_top"),
        "cutout");
    trapdoorBlockWithRenderType(
        ((TrapDoorBlock) ModBlocks.SAPPHIRE_TRAPDOOR.get()),
        modLoc("block/sapphire_trapdoor"),
        true,
        "cutout");

    makeStrawberryCrop(
        (CropBlock) ModBlocks.STRAWBERRY_CROP.get(), "strawberry_stage", "strawberry_stage");

    makeCornCrop(((CropBlock) ModBlocks.CORN_CROP.get()), "corn_stage_", "corn_stage_");
  }

  public void makeCornCrop(CropBlock block, String modelName, String textureName) {
    Function<BlockState, ConfiguredModel[]> function =
        state -> cornStates(state, block, modelName, textureName);

    getVariantBuilder(block).forAllStates(function);
  }

  private ConfiguredModel[] cornStates(
      BlockState state, CropBlock block, String modelName, String textureName) {
    ConfiguredModel[] models = new ConfiguredModel[1];
    models[0] =
        new ConfiguredModel(
            models()
                .crop(
                    modelName + state.getValue(((CornCropBlock) block).getAgeProperty()),
                    ResourceLocation.fromNamespaceAndPath(
                        MyMod.MODID,
                        "block/"
                            + textureName
                            + state.getValue(((CornCropBlock) block).getAgeProperty())))
                .renderType("cutout"));

    return models;
  }

  public void makeStrawberryCrop(CropBlock block, String modelName, String textureName) {
    Function<BlockState, ConfiguredModel[]> function =
        state -> strawberryStates(state, block, modelName, textureName);

    getVariantBuilder(block).forAllStates(function);
  }

  private ConfiguredModel[] strawberryStates(
      BlockState state, CropBlock block, String modelName, String textureName) {
    ConfiguredModel[] models = new ConfiguredModel[1];
    models[0] =
        new ConfiguredModel(
            models()
                .crop(
                    modelName + state.getValue(((StrawberryCropBlock) block).getAgeProperty()),
                    ResourceLocation.fromNamespaceAndPath(
                        MyMod.MODID,
                        "block/"
                            + textureName
                            + state.getValue(((StrawberryCropBlock) block).getAgeProperty())))
                .renderType("cutout"));

    return models;
  }

  private void blockWithItem(RegistryObject<Block> block) {
    simpleBlockWithItem(block.get(), cubeAll(block.get()));
  }
}
