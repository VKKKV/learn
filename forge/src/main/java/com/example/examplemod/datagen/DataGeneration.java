package com.example.examplemod.datagen;

import com.example.examplemod.MyMod;
import java.util.concurrent.CompletableFuture;
import net.minecraft.core.HolderLookup;
import net.minecraft.data.DataGenerator;
import net.minecraft.data.PackOutput;
import net.minecraftforge.common.data.ExistingFileHelper;
import net.minecraftforge.data.event.GatherDataEvent;

public class DataGeneration {

  public static void generate(GatherDataEvent event) {
    String modId = MyMod.MODID;

    DataGenerator generator = event.getGenerator();
    PackOutput packOutput = generator.getPackOutput();
    ExistingFileHelper existingFileHelper = event.getExistingFileHelper();
    CompletableFuture<HolderLookup.Provider> lookupProvider = event.getLookupProvider();

    generator.addProvider(event.includeClient(), new Recipes(packOutput));
    generator.addProvider(event.includeClient(), new LootTables(packOutput));
    generator.addProvider(event.includeClient(), new BlockStates(packOutput, existingFileHelper));
    generator.addProvider(event.includeClient(), new ItemModels(packOutput, existingFileHelper));

    BlockTagsGenerator blockTagGenerator =
        generator.addProvider(
            event.includeServer(),
            new BlockTagsGenerator(packOutput, lookupProvider, existingFileHelper));
    generator.addProvider(
        event.includeServer(),
        new ModItemTags(
            packOutput, lookupProvider, blockTagGenerator.contentsGetter(), existingFileHelper));

    generator.addProvider(event.includeServer(), new ModGlobalLootModifiers(packOutput, modId));

    generator.addProvider(
        event.includeServer(),
        new ModPoiTypeTags(packOutput, lookupProvider, modId, existingFileHelper));
  }
}
