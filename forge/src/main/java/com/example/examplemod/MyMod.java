package com.example.examplemod;

import com.example.examplemod.block.ModBlocks;
import com.example.examplemod.block.entity.ModBlockEntities;
import com.example.examplemod.datagen.DataGeneration;
import com.example.examplemod.entity.ModEntities;
import com.example.examplemod.entity.client.RhinoRenderer;
import com.example.examplemod.item.ModCreativeModTabs;
import com.example.examplemod.item.ModItems;
import com.example.examplemod.loot.ModLootModifiers;
import com.example.examplemod.recipe.ModRecipes;
import com.example.examplemod.screen.GemPolishingStationScreen;
import com.example.examplemod.screen.ModMenuTypes;
import com.example.examplemod.sound.ModSounds;
import com.example.examplemod.villager.ModVillagers;
import com.mojang.logging.LogUtils;
import net.minecraft.client.gui.screens.MenuScreens;
import net.minecraft.client.renderer.entity.EntityRenderers;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.FlowerPotBlock;
import net.minecraftforge.api.distmarker.Dist;
import net.minecraftforge.eventbus.api.IEventBus;
import net.minecraftforge.eventbus.api.SubscribeEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.event.lifecycle.FMLClientSetupEvent;
import net.minecraftforge.fml.event.lifecycle.FMLCommonSetupEvent;
import net.minecraftforge.fml.javafmlmod.FMLJavaModLoadingContext;
import org.slf4j.Logger;

@Mod(MyMod.MODID)
public class MyMod {
  public static final String MODID = "mymod";
  private static final Logger LOGGER = LogUtils.getLogger();

  public MyMod(FMLJavaModLoadingContext context) {
    IEventBus modEventBus = context.getModEventBus();

    ModItems.register(modEventBus);
    ModBlocks.register(modEventBus);
    ModCreativeModTabs.register(modEventBus);
    ModLootModifiers.register(modEventBus);
    ModVillagers.register(modEventBus);
    ModSounds.register(modEventBus);
    ModEntities.register(modEventBus);

    ModBlockEntities.register(modEventBus);
    ModMenuTypes.register(modEventBus);

    ModRecipes.register(modEventBus);

    modEventBus.addListener(this::commonSetup);
    modEventBus.addListener(DataGeneration::generate);
  }

  private void commonSetup(final FMLCommonSetupEvent event) {
    LOGGER.info("HELLO MYMOD");

    event.enqueueWork(
        () -> {
          ((FlowerPotBlock) Blocks.FLOWER_POT)
              .addPlant(ModBlocks.CATMINT.getId(), ModBlocks.POTTED_CATMINT);
        });
  }

  // You can use EventBusSubscriber to automatically register all static methods in the class
  // annotated with @SubscribeEvent
  @Mod.EventBusSubscriber(modid = MODID, bus = Mod.EventBusSubscriber.Bus.MOD, value = Dist.CLIENT)
  public static class ClientModEvents {
    @SubscribeEvent
    public static void onClientSetup(FMLClientSetupEvent event) {
      EntityRenderers.register(ModEntities.RHINO.get(), RhinoRenderer::new);

      MenuScreens.register(ModMenuTypes.GEM_POLISHING_MENU.get(), GemPolishingStationScreen::new);
    }
  }
}
